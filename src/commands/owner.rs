use crate::{
  common::{
    db::trees::emojis,
    msg::{
      channel_message, direct_message
    },
    system
  },
  steins::{ gate
          , ai::cache::{ ACTIVITY_LEVEL
                       , actualize_cache
                       , clear_cache }
          }
};

use serenity::{
  prelude::*,
  model::channel::Message,
  model::id::ChannelId,
  framework::standard::{
    Args, CommandResult,
    macros::command
  }
};

use std::sync::atomic::Ordering;

#[command]
#[min_args(2)]
#[owners_only]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  if let Ok(property) = args.single::<String>() {
    #[allow(clippy::single_match)]
    match property.as_str() {
      "activity" =>
        if let Ok(level) = args.single::<u32>() {
          ACTIVITY_LEVEL.store(level, Ordering::Relaxed);
          let chan_msg = format!("Activity level is: {} now", level);
          channel_message(&ctx, &msg, &chan_msg).await;
        },
      _ => ()
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[owners_only]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  let last_channel_u64 = gate::LAST_CHANNEL.load(Ordering::Relaxed);
  if last_channel_u64 != 0 {
    let last_channel_conf = ChannelId( last_channel_u64 );
    if msg.guild_id.is_some() {
      let text = args.message();
      if !text.is_empty() {
        if let Err(why) = last_channel_conf.say(ctx, text).await {
          error!("Failed say {:?}", why);
        }
      }
    }
  }
  Ok(())
}

#[command]
#[owners_only]
async fn clear_messages(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if args.len() == 1 {
    let countdown: u64 = args.find().unwrap_or_default();
    if let Ok(vec) = msg.channel_id.messages(ctx, |g| g.before(msg.id).limit(countdown)).await {
      let mut vec_id = Vec::new();
      for message in vec {
        vec_id.push(message.id);
      }
      vec_id.push(msg.id);
      match msg.channel_id.delete_messages(ctx, vec_id.as_slice()).await {
        Ok(val)  => val,
        Err(_err) => (),
      };
    }
    direct_message(ctx, &msg, &format!("Deleted {} messages", countdown)).await;
  } else if args.len() == 2 {
    let countdown: usize = args.find().unwrap_or_default();
    let counter: usize = args.find().unwrap_or_default();
    let full = countdown + counter;
    if let Ok(vec) = msg.channel_id.messages(ctx, |g| g.before(msg.id).limit(full as u64)).await {
      let mut vec_id = Vec::new();
      for (i, message) in vec.iter().rev().enumerate() {
        if i < countdown {
          vec_id.push(message.id);
        }
      }
      vec_id.push(msg.id);
      match msg.channel_id.delete_messages(ctx, vec_id.as_slice()).await {
        Ok(val)  => val,
        Err(_err) => (),
      };
    }
    direct_message(ctx, &msg, &format!("Deleted {} messages", countdown)).await;
  }
  Ok(())
}

#[command]
#[owners_only]
async fn clear_chain_cache(ctx: &Context, msg: &Message) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  clear_cache().await;
  channel_message(&ctx, &msg, "Cache removed").await;
  Ok(())
}

#[command]
#[owners_only]
async fn update_cache(ctx: &Context, msg: &Message) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  actualize_cache(ctx, true).await;
  channel_message(ctx, msg, "Cache updated").await;
  Ok(())
}

#[command]
#[owners_only]
async fn upgrade(ctx: &Context, msg: &Message) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  if let Err(why) = system::upgrade::upgrade_amadeus(ctx, &msg.channel_id).await {
    error!("Failed to upgrade Amadeus {:?}", why);
  }
  Ok(())
}

#[command]
#[owners_only]
async fn twitch_token_update(ctx: &Context, msg: &Message) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  if system::hacks::twitch_update(ctx).await.is_ok() {
    channel_message(&ctx, &msg, "twitch access token updated").await;
  }
  Ok(())
}

#[command]
#[owners_only]
#[min_args(3)]
async fn register_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  if let Some(guild_id) = msg.guild_id {
    let message_id = args.single::<u64>()?;
    let emoji_id = args.single::<u64>()?;
    let role_id = args.single::<u64>()?;
    emojis::register_message( guild_id.as_u64()
                            , &message_id
                            , &emoji_id
                            , &role_id ).await;
    direct_message(ctx, &msg, &format!("Message role {} registered", role_id)).await;
  }
  Ok(())
}

#[command]
#[owners_only]
#[min_args(1)]
async fn list_message_roles(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if let Err(why) = msg.delete(ctx).await {
    error!("Error deleting original command {:?}", why);
  }
  if let Some(guild_id) = msg.guild_id {
    let message_id = args.single::<u64>()?;
    if let Ok(Some(mr)) = emojis::message_roles( guild_id.as_u64()
                                               , &message_id ).await {
      channel_message(&ctx, &msg, &format!("message roles: {:?}", mr)).await;
    }
  }
  Ok(())
}
