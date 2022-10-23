use super::{send_embed, Command, EVERYONE_CHECK};
use crate::commands::common::GENERAL_GROUP;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "all commands"]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut all = String::new();
    for command in GENERAL_GROUP.options.commands.iter() {
        all.push_str(&format!(
            "!**{}** - {}",
            command.options.names[0],
            command.options.desc.unwrap()
        ));
    }

    send_embed(
        msg,
        ctx,
        Command {
            title: "Commands",
            description: &all,
        },
    )
    .await?;

    Ok(())
}
#[group]
#[checks(Everyone)]
#[commands(commands)]
struct Help;
