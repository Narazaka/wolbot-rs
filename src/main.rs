mod commands;
mod config;
mod handler;

use commands::help::*;
use commands::wol::*;
use handler::*;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::prelude::Client;
use serenity::prelude::GatewayIntents;

#[group]
#[commands(wol)]
struct General;

#[tokio::main]
async fn main() {
    let config = config::read("config.json").expect("cannot read config.json");
    // コマンド系はすべて大文字にする
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(
        &config.token,
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
