use crate::{
  types::serenity::IContext,
  steins::ai::bert::{
    TRANSLATION_LIMIT,
    en2ru, ru2en
  }
};

use serenity::{
  prelude::*,
  model::channel::*,
  framework::standard::{
    CommandResult, Args,
    macros::command
  },
};

use rust_bert::pipelines::translation::{ Language
                                       , TranslationModelBuilder };

use tokio::task;

async fn bert_translate( ctx: &Context
                       , text: String
                       , source_lang: Language
                       , target_lang: Language ) -> anyhow::Result<String> {
  static RUEN_LANGS: &[Language; 2] = &[Language::Russian, Language::English];
  if RUEN_LANGS.contains(&source_lang) && RUEN_LANGS.contains(&target_lang) {
    let lsm = {
      let data = ctx.data.read().await;
      if let Some(icontext) = data.get::<IContext>() {
        icontext.lazy_static_models
      } else { false }
    };
    if source_lang == Language::Russian {
      ru2en(text, lsm).await
    } else {
      en2ru(text, lsm).await
    }
  } else {
    task::spawn_blocking(move || {
      let mut something = text;
      if something.len() > TRANSLATION_LIMIT {
        if let Some((i, _)) = something.char_indices().rev().nth(TRANSLATION_LIMIT) {
          something = something[i..].to_string();
        }
      }

      let model = TranslationModelBuilder::new()
          .with_source_languages(vec![source_lang])
          .with_target_languages(vec![target_lang])
          .with_device(tch::Device::cuda_if_available())
          .create_model()?;

      let output = model.translate(&[something.as_str()], Some(source_lang), target_lang)?;

      if output.is_empty() {
        Err(anyhow!("Failed to translate with TranslationConfig EnglishToRussian"))
      } else {
        let translation = &output.join(" ");
        Ok(translation.clone())
      }
    }).await?
  }
}

#[command]
#[min_args(1)]
#[description("Translate English to Russian")]
#[aliases(перевод, en2ru)]
pub async fn perevod(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **English** to **Russian**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::English, Language::Russian).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate Ukrainian to Russian")]
pub async fn ua2ru(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **Ukrainian** to **Russian**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::Ukrainian, Language::Russian).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate Russian to English")]
#[aliases(ru2en)]
pub async fn translate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **Russian** to **English**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::Russian, Language::English).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate Russian to Ukrainian")]
pub async fn ru2ua(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **Russian** to **Ukrainian**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::Russian, Language::Ukrainian).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate English to German")]
async fn en2de(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **English** to **German**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::English, Language::German).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate German to English")]
async fn de2en(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **German** to **English**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::German, Language::English).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate English to French")]
async fn en2fr(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **English** to **French**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::English, Language::French).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}

#[command]
#[min_args(1)]
#[description("Translate French to English")]
async fn fr2en(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let text = args.message().to_string();
  if let Err(why) = msg.delete(&ctx).await {
    error!("Error deleting original command, {why}");
  }
  let fields = vec![
    ("Text", format!("{text}\n"), false),
  ];
  let mmm = msg.channel_id.send_message(ctx, |m|
            m.embed(|e|
             e.title("Translating From **French** to **English**...")
              .fields(fields)
              .author(|a| a.icon_url(&msg.author.face())
                           .name(&msg.author.name)
                      )
            )
          ).await;
  match bert_translate(ctx, text.clone(), Language::French, Language::English).await {
    Ok(out) => {
      let fields = vec![
        ("Text", format!("{text}\n"), false),
        ("Translation", out, false)
      ];
      if let Ok(mut mm) = mmm {
        mm.edit(ctx, |m|
          m.embed(|e|
            e.fields(fields)
             .author(|a| a.icon_url(&msg.author.face())
                          .name(&msg.author.name)
                    )
          )
        ).await?;
      }
    },
    Err(why) => {
      error!("Translation failed: {why}");
    }
  }
  Ok(())
}
