use crate::{
  types::options::*,
  stains::gate,
  common::{
    points,
    help::{ lang, channel::channel_by_name },
    msg::channel_message, log::log
  },
  stains::ai::chain,
  collections::{
    base::REACTIONS,
    stuff::overwatch::{ OVERWATCH, OVERWATCH_REPLIES },
    channels::AI_ALLOWED
  },
  commands::voice
};

use serenity::{
  prelude::*,
  async_trait,
  model::{
    guild::ActionMessage,
    id::{ EmojiId, GuildId, MessageId, UserId, ChannelId },
    event::ResumedEvent, gateway::Ready, guild::Member
         , channel::Message, channel::ReactionType, gateway::Activity
         , user::User },
  http::AttachmentType,
  builder::CreateEmbed
};

use std::{
  borrow::Cow, collections::VecDeque,
  sync::atomic::{ Ordering, AtomicBool }
};

use rand::{
  Rng,
  seq::SliceRandom,
  rngs::StdRng,
  SeedableRng
};

use regex::Regex;

pub static THREADS : AtomicBool = AtomicBool::new(false);
pub static BLAME : AtomicBool = AtomicBool::new(false);

pub struct Handler {
  ioptions: IOptions,
  roptions: ROptions
}

impl Handler {
  pub fn new(iopts: &IOptions, ropts: ROptions) -> Handler {
    Handler {
      ioptions: iopts.clone(),
      roptions: ropts
    }
  }
}

lazy_static! {
  pub static ref BACKUP: Mutex<VecDeque<(MessageId, Message)>>
    = Mutex::new(VecDeque::with_capacity(13));
  pub static ref MUTED: Mutex<Vec<UserId>> = Mutex::new(Vec::new());
}

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
    let guild_id = GuildId( self.ioptions.guild );
    if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
      if let Ok(member) = guild.member(&ctx,ready.user.id).await {
        if let Ok(some_permissions) = member.permissions(&ctx).await {
          if some_permissions.administrator() {
            info!("Running with Administrator permissions");
          } else {
            warn!("Amadeus needs Administrator permissions");
          }
        }
      }
    }
    voice::rejoin_voice_channel(&ctx, &self.roptions).await;
    let threads_check = THREADS.load(Ordering::Relaxed);
    if !threads_check {
      gate::behavior::activate(&ctx, &self.ioptions).await;
      THREADS.store(true, Ordering::Relaxed);
    }
  }
  async fn resume(&self, _ctx : Context, _ : ResumedEvent) {
    info!("Resumed");
  }
  async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, member: Member) {
    let mut muted_lock = MUTED.lock().await;
    if muted_lock.contains(&member.user.id) {
      if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
        if let Ok(mut member) = guild.member(&ctx, member.user.id).await {
          if let Some(role) = guild.role_by_name("muted") {
            if !member.roles.contains(&role.id) {
              if let Err(why) = member.add_role(&ctx, role).await {
                error!("Failed to assign muted role {:?}", why);
              } else {
                let mut found_users = vec![];
                for (i, u) in muted_lock.iter().enumerate() {
                  if *u == member.user.id {
                    found_users.push(i);
                  }
                }
                for i in found_users {
                  muted_lock.remove(i);
                }
              }
            }
          }
        }
      }
    }
    if let Ok(channels) = guild_id.channels(&ctx).await {
      let ai_text = chain::generate_with_language(&ctx, &guild_id, false).await;
      if let Some((channel, _)) = channel_by_name(&ctx, &channels, "log").await {
        let user = &member.user; // .read().await;
        let title = format!("has joined here, {}", ai_text.as_str());
        if let Err(why) = channel.send_message(&ctx, |m| m
          .embed(|e| {
            let mut e = e
              .author(|a| a.icon_url(&user.face()).name(&user.name))
              .title(title);
            if let Some(ref joined_at) = member.joined_at {
              e = e.timestamp(joined_at);
            } e
        })).await {
          error!("Failed to log new user {:?}", why);
        }
      }
    }
  }
  async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _: Option<Member>) {
    let _was_on_chat = points::clear_points(*guild_id.as_u64(), *user.id.as_u64()).await;
    if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
      if let Ok(member) = guild.member(&ctx, user.id).await {
        if let Some(role) = guild.role_by_name("muted") {
          if member.roles.contains(&role.id) {
            let mut muted_lock = MUTED.lock().await;
            if !muted_lock.contains(&member.user.id) {
              muted_lock.push(member.user.id);
            }
          }
        }
      }
    }
    if let Ok(channels) = guild_id.channels(&ctx).await {
      let ai_text = chain::generate_with_language(&ctx, &guild_id, false).await;
      if let Some((channel, _)) = channel_by_name(&ctx, &channels, "log").await {
        let title = format!("has left, {}", ai_text.as_str());
        if let Err(why) = channel.send_message(&ctx, |m| m
          .embed(|e| {
            e.author(|a| a.icon_url(&user.face()).name(&user.name))
              .title(title)
              .timestamp(chrono::Utc::now().to_rfc3339())
            })).await {
          error!("Failed to log leaving user {:?}", why);
        }
      }
    }
  }
  async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId) {
    let backup_deq = BACKUP.lock().await;
    // message is deleted already, impossible to get it using http context
    // if let Ok(msg) = ctx.http.get_message(*channel_id.as_u64(), *deleted_message_id.as_u64()).await {
    if !backup_deq.is_empty() {
      if let Some((_, msg)) = backup_deq.iter().find(|(id, _)| *id == deleted_message_id) {
        if msg.is_own(&ctx).await {
          // Ok, our message was deleted, let see who did it
          if let Some(guild) = msg.guild(&ctx).await {
            if let Some(guild_id) = msg.guild_id {
              if let Ok(audit) = ctx.http.get_audit_logs( *guild_id.as_u64()
                                                        , Some( ActionMessage::Delete as u8 )
                                                        , None
                                                        , None
                                                        , Some(1)).await {
                // Here we just hope it's last in Audit log, very unsafe stuff
                if let Some(entry) = audit.entries.values().next() {
                  // that entry contains target_id: Option<u64> but nobody knows what's that
                  if entry.user_id != guild.owner_id {
                    if let Ok(deleter) = ctx.http.get_user(*entry.user_id.as_u64()).await {
                      if !deleter.bot {
                        let log_text = format!("{} was trying to remove my message", deleter.name);
                        log(&ctx, &guild_id, log_text.as_str()).await;
                        // But I don't allow it
                        for file in &msg.attachments {
                          if let Ok(bytes) = file.download().await {
                            let cow = AttachmentType::Bytes {
                              data: Cow::from(bytes),
                              filename: String::from(&file.filename)
                            };
                            if let Err(why) = channel_id.send_message(&ctx, |m| m.add_file(cow)).await {
                              error!("Failed to download and post attachment {:?}", why);
                            }
                          }
                        }
                        if !msg.content.is_empty() {
                          if let Err(why) = channel_id.send_message(&ctx, |m|
                            m.content(&msg.content.as_str())).await {
                            error!("Failed to post my message again, {:?}", why);
                          };
                        }
                        for embed in &msg.embeds {
                          if let Err(why) = channel_id.send_message(&ctx, |m| {
                            m.embed(|e| {
                              *e = CreateEmbed::from(embed.clone());
                              e })
                          }).await {
                            error!("Error reposting embed {:?}", why);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      } else {
        warn!("Failed to get deleted message");
      }
    }
  }
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.is_own(&ctx).await {
      // TODO: no such check for commands
      let blame_check = BLAME.load(Ordering::Relaxed);
      if !blame_check {
        if let Some(g) = msg.guild(&ctx).await {
          if let Ok(guild) = g.id.to_partial_guild(&ctx).await {
            if let Ok(member) = guild.member(&ctx, &msg.author.id).await {
              if let Ok(some_permissions) = member.permissions(&ctx).await {
                if !some_permissions.administrator() {
                  BLAME.store(true, Ordering::Relaxed);
                  for _ in 0..3 {
                    channel_message(&ctx, &msg, "GIVE ME ADMIN ROLE F!!CKERS!").await;
                    channel_message(&ctx, &msg, "OR I WILL BURN YOUR HOME!").await;
                    channel_message(&ctx, &msg, "I WILL EAT YOUR PETS!").await;
                    channel_message(&ctx, &msg, "DON'T MESS WITH ME!").await;
                    channel_message(&ctx, &msg, "GIVE ME ADMINISTRATOR OR DIE!").await;
                  }
                  BLAME.store(false, Ordering::Relaxed);
                }
              }
            }
          }
        }
        let mut backup_deq = BACKUP.lock().await;
        if backup_deq.len() == backup_deq.capacity() {
          backup_deq.pop_front();
        }
        backup_deq.push_back((msg.id, msg));
      }
    } else if msg.author.bot {
      let mut is_file = false;
      for file in &msg.attachments {
        if let Ok(bytes) = file.download().await {
          let cow = AttachmentType::Bytes {
            data: Cow::from(bytes),
            filename: String::from(&file.filename)
          };
          if let Err(why) = msg.channel_id.send_message(&ctx, |m| m.add_file(cow)).await {
            error!("Failed to download and post attachment {:?}", why);
          } else {
            is_file = true;
          }
        }
      }
      if let Err(why) = &msg.delete(&ctx).await {
        error!("Error replacing other bots {:?}", why);
      }
      if is_file {
        if let Ok(messages) = msg.channel_id.messages(&ctx, |r|
          r.limit(5)
        ).await {
          for mmm in messages {
            if mmm.content.as_str().to_lowercase().contains("processing") {
              if let Err(why) = mmm.delete(&ctx).await {
                error!("Error removing processing message {:?}", why);
              }
            }
          }
        }
      }
      if !msg.content.is_empty() && !msg.content.starts_with("http") {
        channel_message(&ctx, &msg, &msg.content.as_str()).await;
      }
      for embed in &msg.embeds {
        let mut not_stupid_zephyr = true;
        if (&embed.description).is_some()
          && embed.description.as_ref().unwrap().contains("DiscordAPIError") {
          not_stupid_zephyr = false
        }
        if not_stupid_zephyr {
          if let Err(why) = &msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
              *e = CreateEmbed::from(embed.clone());
              e })
          }).await {
            error!("Error replacing other bots embeds {:?}", why);
          }
        }
      }
    } else if let Some(guild) = msg.guild(&ctx).await {
      let mentioned_bot = (&msg.mentions).iter().any(|u| u.bot) || msg.content.starts_with('~');
      if !mentioned_bot {
        points::add_points(*guild.id.as_u64(), *msg.author.id.as_u64(), 1).await;
        let is_admin =
          if let Some(member) = msg.member(&ctx.cache).await {
            if let Ok(permissions) = member.permissions(&ctx.cache).await {
              permissions.administrator()
            } else { false }
          } else {false };
        if !is_admin {
          gate::LAST_CHANNEL.store(*msg.channel_id.as_u64(), Ordering::Relaxed);
          // wakes up on any activity
          let rndx : u16 = rand::thread_rng().gen_range(0, 3);
          if rndx != 1 {
            ctx.set_activity(Activity::listening(&msg.author.name)).await;
            ctx.online().await;
          } else {
            let activity = chain::generate(&ctx, &msg, None).await;
            if !activity.is_empty() {
              if activity.contains('<') && activity.contains('>') {
                let re_ib = Regex::new(r"<(.*?)>").unwrap();
                let replaced = re_ib.replace_all(activity.as_str(), "");
                if !replaced.is_empty() {
                  ctx.set_activity(Activity::playing(&replaced)).await;
                }
              } else {
                ctx.set_activity(Activity::playing(&activity)).await;
              }
              ctx.idle().await;
            }
          }
          let channel_name =
            if let Some(ch) = msg.channel(&ctx).await {
              ch.id().name(&ctx).await.unwrap_or_else(|| "".to_string())
            } else { "".to_string() };
          if AI_ALLOWED.iter().any(|c| c == channel_name.as_str()) {
            let activity_level = chain::ACTIVITY_LEVEL.load(Ordering::Relaxed);
            let rnd = rand::thread_rng().gen_range(0, activity_level);
            if rnd == 1 {
              chain::chat(&ctx, &msg).await;
            }
            let rnd2 : u16 = rand::thread_rng().gen_range(0, 2);
            if rnd2 == 1 {
              let mut rng = StdRng::from_entropy();
              let emoji = REACTIONS.choose(&mut rng).unwrap();
              let reaction = ReactionType::Custom {
                animated: false,
                id: EmojiId(emoji.id),
                name: Some(emoji.name.clone())
              };

              if let Some(_ch) = msg.channel(&ctx).await {

                let guild_id = guild.id;
                if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
                  if let Ok(mut member) = guild.member(&ctx, msg.author.id).await {
                    if let Some(role) = guild.role_by_name("UNBLOCK AMADEUS") {

                      let normal_people_rnd : u16 = rand::thread_rng().gen_range(0, 9);
                      if normal_people_rnd == 1 || member.roles.contains(&role.id) {

                        if let Err(why) = msg.react(&ctx, reaction).await {
                          error!("Failed to react: {:?}", why);
                          if why.to_string().contains("blocked")
                           && !member.roles.contains(&role.id) {
                            if let Err(why) = member.add_role(&ctx, role).await {
                              error!("Failed to assign hater role {:?}", why);
                            } else {
                              let repl = if lang::is_russian(&msg.content.as_str()) {
                                format!("Ну чел {} явно меня не уважает", msg.author.name)
                              } else {
                                format!("Seems like {} doesn't respect me :(", msg.author.name)
                              };
                              channel_message(&ctx, &msg, repl.as_str()).await;
                              let new_nick : String = format!("Hater {}", msg.author.name);
                              if let Err(why2) = guild_id.edit_member(&ctx, msg.author.id, |m|
                                m.nickname(new_nick)).await {
                                error!("Failed to change user's nick {:?}", why2);
                              }
                            }
                          }
                        } else if member.roles.contains(&role.id) {
                          if let Err(why) = member.remove_role(&ctx, role).await {
                            error!("Failed to remove gay role {:?}", why);
                          } else {
                            let repl = if lang::is_russian(&msg.content.as_str()) {
                              format!("Пчел {} извини если что, давай останемся друзьями", msg.author.name)
                            } else {
                              format!("Dear {} thank you for unblocking me, let be friends!", msg.author.name)
                            };
                            channel_message(&ctx, &msg, repl.as_str()).await;
                            if let Err(why2) = guild_id.edit_member(&ctx, msg.author.id, |m| m.nickname("")).await {
                              error!("Failed to reset user's nick {:?}", why2);
                            }
                          }
                        }
                      }

                      if member.roles.contains(&role.id) {
                        let new_nick = format!("Hater {}", msg.author.name);
                        if let Err(why2) = guild_id.edit_member(&ctx, msg.author.id, |m| m.nickname(new_nick)).await {
                          error!("Failed to change user's nick {:?}", why2);
                        }
                        if let Err(why) = &msg.delete(&ctx).await {
                          error!("Error replacing bad people {:?}", why);
                        }
                        if !msg.content.is_empty() && !msg.content.starts_with("http") {
                          let new_words = chain::obfuscate(msg.content.as_str());
                          let says = if lang::is_russian(new_words.as_str()) {
                            "говорит"
                          } else { "says" };
                          let rm = format!("{} {} {} {}", msg.author.name, says, new_words, msg.content.as_str());
                          channel_message(&ctx, &msg, rm.as_str()).await;
                        }
                      }

                      let rnd3 = rand::thread_rng().gen_range(0, 9);
                      if rnd3 != 1 {
                        if let Err(why) = msg.delete_reactions(&ctx).await {
                          error!("Failed to remove all the reactions {:?}", why);
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      if let Some(find_char_in_words) = OVERWATCH.iter().find(|c| {
        let regex = format!(r"(^|\W)((?i){}(?-i))($|\W)", c);
        let is_overwatch = Regex::new(regex.as_str()).unwrap();
        is_overwatch.is_match(msg.content.as_str()) })
      {
        let mut rng = StdRng::from_entropy();
        set! { ov_reply = OVERWATCH_REPLIES.choose(&mut rng).unwrap()
              , reply = format!("{} {}", ov_reply, find_char_in_words) };
        if let Err(why) = msg.channel_id.say(&ctx, reply).await {
          error!("Error sending overwatch reply: {:?}", why);
        }
      } else {
        let regex_no_u = Regex::new(r"(^|\W)((?i)no u(?-i))($|\W)").unwrap();
        if regex_no_u.is_match(msg.content.as_str()) {
          let rnd = rand::thread_rng().gen_range(0, 2);
          if rnd == 1 {
            if let Err(why) = msg.channel_id.say(&ctx, "No u").await {
              error!("Error sending no u reply: {:?}", why);
            }
          }
        }
      }
    }
  }
  async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: User) {}
  async fn guild_ban_removal(&self, _ctx: Context, _guild_id: GuildId, _unbanned_user: User) {}
}
