use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Help"]
#[strikethrough_commands_tip_in_guild = ""]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
