use crate::collections::team::DISCORDS;

use serenity::{
  prelude::*,
  model::{ id::GuildId
         , channel::Message,
         }
};

use regex::Regex;
use once_cell::sync::Lazy;

static SLUR: Lazy<Regex> =
  Lazy::new(||
    Regex::new(
      r"(fag(g|got|tard)?\b|cunt|cock\s?sucker(s|ing)?|ni((g{2,}|q)+|[gq]{2,})[ae3r]+(s|z)?s?)"
    ).unwrap());

static LIGHT_SLUR_LOL: Lazy<Regex> =
    Lazy::new(||
      Regex::new(
        r"(bitch(es|ing|y)?|whor(es?|ing)|(bas|re)tard(ed)?s?)"
      ).unwrap());

async fn delete( guild_id: &GuildId
               , ctx: &Context
               , msg: &Message
               , disable_communication: bool
               , really_delete: bool
               , reason: &str
               ) {
  if disable_communication {
    if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
      if let Ok(mut member) = guild.member(&ctx, msg.author.id).await {
        let timeout = chrono::Utc::now() + chrono::Duration::days(1);
        if let Err(why) = member.disable_communication_until_datetime(ctx, timeout).await {
          error!("Failed to timeout user for a day {why}");
        }
      }
    }
  }
  if let Some(ds) = DISCORDS.get(&guild_id.0) {
    if let Some(log) = ds.log {
      if let Err(why) = log.send_message(&ctx, |m| m
        .embed(|e| {
          e.author(|a| a.icon_url(&msg.author.face()).name(&msg.author.name))
           .title(reason)
           .description(&format!( "User UID: {}\n original message: {}"
                                , msg.author.id.0, &msg.content))
           .timestamp(chrono::Utc::now().to_rfc3339())
        })).await {
        error!("Failed to log leaving user {why}");
      }
    }
  }
  if really_delete {
    if let Err(why) = &msg.delete(&ctx).await {
      error!("Error deleting spam {why}");
    }
    if let Err(why) =
      msg.author.direct_message(ctx, |m|
        m.content(&format!("your message was removed with reason: {reason}\n please contact moderators if you think it was done by mistake"))
      ).await {
      error!("Error sending message from spam blocker {why}");
    }
  } else {
    if let Err(why) =
      msg.author.direct_message(ctx, |m|
        m.content(&format!("please, try to avoid using bad words"))
      ).await {
      error!("Error sending message from spam blocker {why}");
    }
  }
}

pub async fn spam_check(
      guild_id: &GuildId
    , ctx: &Context
    , msg: &Message) {
  let lowercase = msg.content.to_lowercase();
  if lowercase.contains("disocrds.gift") {
    delete( guild_id, ctx, msg, true, true
          , "SCAM MESSAGE BLOCKED" ).await;
  }
  for embed in &msg.embeds {
    if let Some(url) = &embed.url {
      if url.contains("disocrds.gift") {
        delete( guild_id, ctx, msg, true, true
              , "SCAM MESSAGE BLOCKED" ).await;
      }
    }
  }
  if SLUR.is_match(&lowercase) {
    delete( guild_id, ctx, msg, false, true
          , "SLUR USED" ).await;
  }
  if LIGHT_SLUR_LOL.is_match(&lowercase) {
    delete( guild_id, ctx, msg, false, false
          , "LIGHT SLUR USED" ).await;
  }
}

#[cfg(test)]
mod antispam_tests {
  use super::*;
  #[test]
  fn slurs_test() {
    assert_eq!(
      SLUR.is_match("retard"), false
    );
    assert_eq!(
      LIGHT_SLUR_LOL.is_match("bastard"), true
    );
    assert_eq!(
      SLUR.is_match("hi nigga"), true
    );
    assert_eq!(
      SLUR.is_match("Hello my friend"), false
    );
  }
}
