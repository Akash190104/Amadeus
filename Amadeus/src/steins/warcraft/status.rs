use crate::{
  types::tracking::{ W3CStats, GameMode },
  common::constants::{ W3C_STATS_ROOM, W3C_STATS_MSG },
  commands::w3c::CURRENT_SEASON,
  steins::warcraft::poller::GAMES
};

use chrono::{
  Timelike,
  Datelike
};
use serenity::prelude::*;

use std::{
  sync::atomic::Ordering::Relaxed,
  collections::HashMap
};

use async_std::fs;

const WEEKLY_STATS_FNAME: &str  = "weekly.yml";

#[derive(Clone, Serialize, Deserialize)]
pub struct WeeklyWinLoses {
  pub wins: u64,
  pub losses: u64
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Weekly {
  pub reset_week: u32,
  pub statistics: HashMap<String, WeeklyWinLoses>
}

pub async fn get_weekly() -> anyhow::Result<Weekly> {
  if !std::path::Path::new(WEEKLY_STATS_FNAME).exists() {
    clear_weekly(chrono::Utc::now().iso_week().week()).await?;
  }
  let contents = fs::read_to_string(WEEKLY_STATS_FNAME).await?;
  let yml = serde_yaml::from_str(&contents)?;
  Ok(yml)
}

pub async fn add_to_weekly(p: &str, win: bool) -> anyhow::Result<()> {
  let mut current_weekly = get_weekly().await?;
  if let Some(p_stats) = current_weekly.statistics.get_mut(p) {
    if win {
      p_stats.wins += 1;
    } else {
      p_stats.losses += 1;
    }
  } else {
    let stats = WeeklyWinLoses {
      wins    : if win { 1 } else { 0 },
      losses  : if win { 0 } else { 1 }
    };
    current_weekly.statistics.insert(p.to_string(), stats);
  }
  let yml = serde_yaml::to_string(&current_weekly)?;
  fs::write(WEEKLY_STATS_FNAME, yml).await?;
  Ok(())
}

async fn clear_weekly(week: u32) -> anyhow::Result<()> {
  let init = Weekly {
    reset_week: week,
    statistics: HashMap::new()
  };
  let yml = serde_yaml::to_string(&init)?;
  fs::write(WEEKLY_STATS_FNAME, yml).await?;
  Ok(())
}

pub async fn status_update(ctx: &Context, stats: &W3CStats) -> anyhow::Result<()> {
  if let Ok(mut statusmsg) = W3C_STATS_ROOM.message(ctx, W3C_STATS_MSG).await {
    let season = CURRENT_SEASON.load(Relaxed);
    let weekly = get_weekly().await?;
    let now = chrono::Utc::now();
    // only check on midnight (just because)
    if now.hour() == 0 {
      let now_week = now.iso_week().week();
      if now_week != weekly.reset_week {
        clear_weekly(now_week).await?;
      }
    }

    let mut tracking_info = vec![];
    { // Games lock scope
      let games_lock = GAMES.lock().await;
      for game in games_lock.values() {
        if let Some(fp) = game.players.first() {
          let name = fp.player.battletag
                       .split('#')
                       .collect::<Vec<&str>>()[0];
          let game_mode_str = match game.mode {
            GameMode::Solo  => "1x1",
            GameMode::Team2 => "2x2",
            GameMode::Team4 => "4x4"
          };
          tracking_info.push(
            format!("{} playing {} game for {} minutes"
            , name
            , game_mode_str
            , game.passed_time)
          );
        }
      }
    }
    let tracking_str = 
      if tracking_info.is_empty() {
        String::from("currently no games")
      } else {
        tracking_info.join("\n")
      };
    let weekly_str =
      if weekly.statistics.is_empty() {
        String::from("no weekly statistic")
      } else {
        let mut weekly_vec = vec![];
        for (p, d) in weekly.statistics {
          let name = p.split('#')
                      .collect::<Vec<&str>>()[0];
          let winrate = ( (d.wins as f32 / (d.wins + d.losses) as f32) * 100.0).round();
          weekly_vec.push(
            format!( "{}: {} wins, {} losses, {}%"
                   , name
                   , d.wins
                   , d.losses
                   , winrate )
          );
        }
        weekly_vec.join("\n")
      };
    let stats_str = format!(
"
__**currently running:**__
```
SOLO GAMES: {}
2x2  GAMES: {}
4x4  GAMES: {}
```

__**currently playing:**__
```
{}
```

__**weekly stats:**__
```
{}
```

__**meta info:**__
```
current season: {}
```"
    , stats.games_solo
    , stats.games_2x2
    , stats.games_4x4
    , tracking_str
    , weekly_str
    , season);
    statusmsg.edit(ctx, |m| m.content("")
             .embed(|e|
              e.color((64, 32, 32))
               .title("☥ Status Grid ☥")
               .description(stats_str)
               .thumbnail("https://vignette.wikia.nocookie.net/steins-gate/images/0/07/Amadeuslogo.png")
               .image("https://vignette.wikia.nocookie.net/steins-gate/images/8/83/Kurisu_profile.png")
               .timestamp(now.to_rfc3339())
    )).await?;
  }
  Ok(())
}
