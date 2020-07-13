use crate::{
  common::{
    help::lang,
    msg::{ reply, channel_message }
  },
  collections::base::{ CONFUSION, CONFUSION_RU, OBFUSCATION, OBFUSCATION_RU },
  collections::channels::AI_LEARN
};

use serenity::{
  prelude::*,
  model::{ channel::{ Message }
         , id::GuildId, id::UserId }
};

use serenity::utils::{
  content_safe,
  ContentSafeOptions,
};

use regex::Regex;

use markov::Chain;

use rand::{
  Rng
};

use std::sync::atomic::{ AtomicU32 };

pub static ACTIVITY_LEVEL : AtomicU32 = AtomicU32::new(66);

pub async fn make_quote(ctx: &Context, msg : &Message, author_id: UserId, limit: u64) -> Option<String> {
  let mut have_something = false;
  if let Some(guild) = msg.guild(&ctx).await {
    let mut chain = Chain::new();
    let re = Regex::new(r"<@!?\d{15,20}>").unwrap();
    let guild_id = guild.id;
    if let Ok(channels) = guild_id.channels(&ctx).await {
      for (chan, _) in channels {
        if let Some(c_name) = chan.name(&ctx).await {
          if AI_LEARN.into_iter().any(|&c| c == c_name.as_str()) {
            if let Ok(messages) = chan.messages(&ctx, |r|
              r.limit(limit)
            ).await {
              for mmm in messages {
                if mmm.author.id == author_id {
                  let mut result = re.replace_all(&mmm.content.as_str(), "").to_string();
                  result = result.replace(": ", "");
                  let is_http = result.starts_with("http") && !result.starts_with("https://images");
                  result =
                    content_safe(&ctx, &result, &ContentSafeOptions::default()
                      .clean_user(false).clean_channel(true)
                      .clean_everyone(true).clean_here(true)).await;
                  if !result.is_empty() && !result.contains("$") && !is_http {
                    chain.feed_str(result.as_str());
                    if !have_something {
                      have_something = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    if have_something {
      return Some(chain.generate_str());
    }
  }
  None
}

pub async fn generate_with_language(ctx: &Context, guild_id: &GuildId, limit: u64, russian : bool) -> String {
  let mut out = String::new();
  if let Ok(channels) = guild_id.channels(&ctx).await {
    let mut chain = Chain::new();
    let re = Regex::new(r"<@!?\d{15,20}>").unwrap();
    for (chan, _) in channels {
      if let Some(c_name) = chan.name(&ctx).await {
        if AI_LEARN.into_iter().any(|&c| c == c_name.as_str()) {
          if let Ok(messages) = chan.messages(&ctx, |r|
            r.limit(limit)
          ).await {
            for mmm in messages {
              if !mmm.author.bot {
                let is_to_bot = mmm.mentions.len() > 0 && (&mmm.mentions).into_iter().any(|u| u.bot);
                if !is_to_bot {
                  let mut result = re.replace_all(&mmm.content.as_str(), "").to_string();
                  result = result.replace(": ", "");
                  let is_http = result.starts_with("http") && !result.starts_with("https://images");
                  result =
                    content_safe(&ctx, &result, &ContentSafeOptions::default()
                      .clean_user(false).clean_channel(true)
                      .clean_everyone(true).clean_here(true)).await;
                  if !result.is_empty() && !result.contains("$") && !is_http {
                    let is_russian = lang::is_russian(result.as_str());
                    if (russian && is_russian) || (!russian && !is_russian) {
                      chain.feed_str(result.as_str());
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    out = chain.generate_str();
  }
  out
}

pub async fn generate_english_or_russian(ctx: &Context, guild_id: &GuildId, limit: u64) -> String {
  let rndx = rand::thread_rng().gen_range(0, 2);
  generate_with_language(&ctx, &guild_id, limit, rndx != 1).await
}

pub async fn generate(ctx: &Context, msg : &Message, limit: u64) -> String {
  let mut out = String::new();
  if let Some(guild) = msg.guild(&ctx).await {
    let mut chain = Chain::new();
    set!{ re = Regex::new(r"<@!?\d{15,20}>").unwrap()
        , msg_content = &msg.content
        , russian = lang::is_russian(msg_content)
        , guild_id = guild.id };
    if let Ok(channels) = guild_id.channels(&ctx).await {
      for (chan, _) in channels {
        if let Some(c_name) = chan.name(&ctx).await {
          if AI_LEARN.into_iter().any(|&c| c == c_name.as_str()) {
            if let Ok(messages) = chan.messages(&ctx, |r|
              r.limit(limit)
            ).await {
              for mmm in messages {
                let mut result = re.replace_all(&mmm.content.as_str(), "").to_string();
                result = result.replace(": ", "");
                let is_http = result.starts_with("http") && !result.starts_with("https://images");
                result =
                  content_safe(&ctx, &result, &ContentSafeOptions::default()
                    .clean_user(false).clean_channel(true)
                    .clean_everyone(true).clean_here(true)).await;
                if !result.is_empty() && !result.contains("$") && !is_http {
                  let is_russian = lang::is_russian(result.as_str());
                  if (russian && is_russian)
                  || (!russian && !is_russian) {
                    chain.feed_str(result.as_str());
                  }
                }
              }
            }
          }
        }
      }
      if !russian {
        for confuse in CONFUSION {
          chain.feed_str( confuse );
        }
      } else {
        for confuse in CONFUSION_RU {
          chain.feed_str( confuse );
        }
      }
      chain.feed_str(msg_content.as_str());
      out = chain.generate_str();
    }
  }
  out
}

pub fn obfuscate(msg_content : &str) -> String {
  let mut chain = Chain::new();
  let russian = lang::is_russian(msg_content);
  if !russian {
    for confuse in OBFUSCATION {
      chain.feed_str( confuse );
    }
  } else {
    for confuse in OBFUSCATION_RU {
      chain.feed_str( confuse );
    }
  }
  chain.feed_str(msg_content);
  chain.generate_str()
}

pub async fn response(ctx: &Context, msg : &Message, limit: u64) {
  let answer = generate(&ctx, &msg, limit).await;
  if !answer.is_empty() {
    reply(&ctx, &msg, answer.as_str()).await;
  }
}

pub async fn chat(ctx: &Context, msg : &Message, limit: u64) {
  let answer = generate(&ctx, &msg, limit).await;
  if !answer.is_empty() {
    let rnd = rand::thread_rng().gen_range(0, 3);
    if rnd == 1 {
      reply(&ctx, &msg, answer.as_str()).await;
    } else {
      channel_message(&ctx, &msg, answer.as_str()).await;
    }
  }
}
