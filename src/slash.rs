use serenity::{
  prelude::*,
  model::{ guild::PartialGuild
         , interactions::ApplicationCommandOptionType
         }
};

pub async fn create_app_commands(ctx: &Context, guild: &PartialGuild) {
  if let Err(why) = guild.create_application_commands(ctx, |cs| {
    cs.create_application_command(|c| c.name("help")
      .description("Display Amadeus Help")
    )
      .create_application_command(|c| c.name("wave")
      .description("Wave a hand you know...")
    )
      .create_application_command(|c| c.name("cry")
      .description("Start to cry!")
    )
      .create_application_command(|c| c.name("cringe")
      .description("just cringe")
    )
      .create_application_command(|c| c.name("ahegao")
      .description("Make an ahegao face")
    )
      .create_application_command(|c| c.name("clap")
      .description("Start clapping")
    )
      .create_application_command(|c| c.name("shrug")
      .description("Shrug shoulders")
    )
      .create_application_command(|c| c.name("lol")
      .description("laugh out loud")
    )
      .create_application_command(|c| c.name("angry")
      .description("Angry feels")
    )
      .create_application_command(|c| c.name("dance")
      .description("Dance Dance Dance")
    )
      .create_application_command(|c| c.name("confused")
      .description("Shows your confusion")
    )
      .create_application_command(|c| c.name("shock")
      .description("If you are shocked")
    )
      .create_application_command(|c| c.name("nervous")
      .description("Feeling nervous")
    )
      .create_application_command(|c| c.name("sad")
      .description("Feeling sad")
    )
      .create_application_command(|c| c.name("happy")
      .description("Feeling happy")
    )
      .create_application_command(|c| c.name("annoyed")
      .description("Really annoyed")
    )
      .create_application_command(|c| c.name("omg")
      .description("Oh my gawd")
    )
      .create_application_command(|c| c.name("smile")
      .description("Do a smile")
    )
      .create_application_command(|c| c.name("ew")
      .description("When you don't like something really")
    )
      .create_application_command(|c| c.name("awkward")
      .description("Feeling awkward")
    )
      .create_application_command(|c| c.name("oops")
      .description("This is just oops emotion...")
    )
      .create_application_command(|c| c.name("lazy")
      .description("Feeling lazy")
    )
      .create_application_command(|c| c.name("hungry")
      .description("Feeling hungry")
    )
      .create_application_command(|c| c.name("stressed")
      .description("Feeling stressed")
    )
      .create_application_command(|c| c.name("scared")
      .description("Really scared")
    )
      .create_application_command(|c| c.name("bored")
      .description("Feeling bored")
    )
      .create_application_command(|c| c.name("yes")
      .description("Yes Yes Yes")
    )
      .create_application_command(|c| c.name("no")
      .description("No No No")
    )
      .create_application_command(|c| c.name("bye")
      .description("Bye Bye")
    )
      .create_application_command(|c| c.name("sorry")
      .description("I am so sorry")
    )
      .create_application_command(|c| c.name("sleepy")
      .description("Feeling sleepy zzz")
    )
      .create_application_command(|c| c.name("wink")
      .description("Close and open one eye quickly")
    )
      .create_application_command(|c| c.name("facepalm")
      .description("A palm of a hand is brought to a face as an expression of dismay")
    )
      .create_application_command(|c| c.name("whatever")
      .description("you don't care")
    )
      .create_application_command(|c| c.name("pout")
      .description("do weird thing with lips")
    )
      .create_application_command(|c| c.name("smug")
      .description("showing an excessive pride in oneself")
    )
      .create_application_command(|c| c.name("smirk")
      .description("smile in an irritatingly smug, conceited, or silly way")
    )
      .create_application_command(|c| c.name("hug")
      .description("Literally hug someone")
      .create_option(|o| {
          o.name("person")
          .description("Person to hug")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("pat")
      .description("Literally pat someone")
      .create_option(|o| {
          o.name("person")
          .description("Person to pat")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("slap")
      .description("Literally slap someone")
      .create_option(|o| {
          o.name("person")
          .description("Person to slap")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("gif")
      .description("Do some specific animation")
      .create_option(|o| {
          o.name("animation")
          .description("Search for specific animation")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("translate")
      .description("Translate Russian to English")
      .create_option(|o| {
          o.name("text")
          .description("What will be translated")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("перевод")
      .description("Перевод с английского на Русский")
      .create_option(|o| {
          o.name("текст")
          .description("Текст для перевода")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("stats")
      .description("Display W3C player statistics")
      .create_option(|o| {
          o.name("battletag")
          .description("Target player")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("борис")
      .description("Команда, которую любит Лилуал")
      .create_option(|o| {
          o.name("текст")
          .description("Текст для Бориса")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("uwu")
      .description("Uwufy some text OwO")
      .create_option(|o| {
          o.name("text")
          .description("Some text...")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("феминизировать")
      .description("Феминизировать предложение")
      .create_option(|o| {
          o.name("текст")
          .description("Текст для феминизации")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("correct")
      .description("Correct grammar for English text")
      .create_option(|o| {
          o.name("text")
          .description("Text for correction")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
      .create_application_command(|c| c.name("time")
      .description("Display current time")
      .create_option(|o| {
          o.name("timezone")
          .description("Optional timezone")
          .kind(ApplicationCommandOptionType::String)
          .required(false)
      })
    )
      .create_application_command(|c| c.name("время")
      .description("Показать текущее время")
      .create_option(|o| {
          o.name("город")
          .description("Дополнительный часовой пояс")
          .kind(ApplicationCommandOptionType::String)
          .required(false)
      })
    )
      .create_application_command(|c| c.name("join")
      .description("Join voice channel with you (you should be in voice channel)")
    )
      .create_application_command(|c| c.name("leave")
      .description("Leave voice channel")
    )
      .create_application_command(|c| c.name("repeat")
      .description("Play last song again")
    )
      .create_application_command(|c| c.name("play")
      .description("Play radio stream or youtube stuff")
      .create_option(|o| {
          o.name("url")
          .description("link for music to play")
          .kind(ApplicationCommandOptionType::String)
          .required(true)
      })
    )
  }).await {
    error!("Failed to register global application commands {:?}", why);
  }
}
