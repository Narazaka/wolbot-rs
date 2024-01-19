use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::read;

#[command]
#[description = "Wake on LAN"]
async fn wol(ctx: &Context, msg: &Message) -> CommandResult {
    let config = read("config.json").expect("cannot read config.json");
    if let Some(channel) = config.get_channel(msg.channel_id.get()) {
        let wol = wakey::WolPacket::from_string(&channel.mac, ':')?;
        if wol.send_magic().is_ok() {
            msg.channel_id.say(&ctx.http, "Waking up...").await?;
        } else {
            msg.channel_id.say(&ctx.http, "no server found...").await?;
        }
    }

    Ok(())
}
