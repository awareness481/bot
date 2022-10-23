use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::*;
use serenity::prelude::*;

async fn send_embed<'a>(
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

struct Command<'a> {
    name: &'a str,
    title: &'a str,
    description: &'a str,
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        name: "!ping",
        title: "Pinging other members",
        description: "Do not ping other people in order to get attention to your question unless they are actively involved in the discussion." }).await?;

    Ok(())
}

#[command]
async fn jobs(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        name: "!jobs",
        title: "Job postings",
        description: "We currently do not allow job posts in this server, unless it's in the context of a discussion. If you're looking to get hired or to advertise a job vacancy see #jobs." }).await?;

    Ok(())
}

#[command]
async fn promotion(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        name: "!promotion",
        title: "Self promotion",
        description: "We have a few channels that allow for self-promotion [#showcase, #community-content]. Sharing promotional links such as referral links, giveaways/contests
        or anything that would be a plain advertisment is discouraged and may be removed." }).await?;

    Ok(())
}

#[command]
async fn dm(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        name: "!DM",
        title: "Direct messages",
        description: "Please do not directly message other members without asking first. If you're looking to get help, it is a lot better to post your question in the applicable channel publicly." }).await?;

    Ok(())
}

#[check]
#[name = "Everyone"]
async fn everyone_check(
    _: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    // Replace 7 with your ID to make this check pass.
    //
    // 1. If you want to pass a reason alongside failure you can do:
    // `Reason::User("Lacked admin permission.".to_string())`,
    //
    // 2. If you want to mark it as something you want to log only:
    // `Reason::Log("User lacked admin permission.".to_string())`,
    //
    // 3. If the check's failure origin is unknown you can mark it as such:
    // `Reason::Unknown`
    //
    // 4. If you want log for your system and for the user, use:
    // `Reason::UserAndLog { user, log }`
    if msg.mention_everyone {
        return Err(Reason::User("Mentions everyone".to_string()));
    }

    Ok(())
}

#[group]
#[checks(Everyone)]
#[commands(ping, jobs, promotion, dm)]
struct General;
