use serenity::{
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::prelude::Message,
    prelude::Context,
};

pub mod common;
pub mod help;
pub struct Command<'a> {
    title: &'a str,
    description: &'a str,
}

pub async fn send_embed<'a>(
    msg: &Message,
    ctx: &Context,
    command: Command<'a>,
) -> Result<serenity::model::channel::Message, serenity::Error> {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(command.title);
                e.description(command.description);
                e
            })
        })
        .await
}

#[check]
#[name = "Everyone"]
pub async fn everyone_check(
    _: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    if msg.mention_everyone {
        return Err(Reason::Log("Mentions everyone".to_string()));
    }

    Ok(())
}
