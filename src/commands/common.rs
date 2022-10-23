use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::{send_embed, Command, EVERYONE_CHECK};

#[command]
#[description = "Information about pinging other members"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        title: "Pinging other members",
        description: "Do not ping other people in order to get attention to your question unless they are actively involved in the discussion." }).await?;

    Ok(())
}

#[command]
#[description = "Rule about job posts"]
async fn jobs(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        title: "Job postings",
        description: "We currently do not allow job posts in this server, unless it's in the context of a discussion. If you're looking to get hired or to advertise a job vacancy see #jobs." }).await?;

    Ok(())
}

#[command]
#[description = "Rule about asking self-promotion"]
async fn promotion(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        title: "Self promotion",
        description: "We have a few channels that allow for self-promotion [#showcase, #community-content]. Sharing promotional links such as referral links, giveaways/contests
        or anything that would be a plain advertisment is discouraged and may be removed." }).await?;

    Ok(())
}

#[command]
#[description = "Rule about directly messaging other members"]
async fn dm(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed(msg, ctx,  Command {
        title: "Direct messages",
        description: "Please do not directly message other members without asking first. If you're looking to get help, it is a lot better to post your question in the applicable channel publicly." }).await?;

    Ok(())
}

#[group]
#[checks(Everyone)]
#[commands(ping, jobs, promotion, dm)]
pub struct General;
