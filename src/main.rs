use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(command) = Command::from_str(&msg.content) {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = send_embed(&msg, &ctx, command).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct Command {
    name: String,
    title: String,
    description: String,
}

impl Command {
    fn from_str(s: &str) -> Option<Command> {
        match matches_message(s) {
            Some("!ping") => Some(Command {
                name: "!ping".to_string(),
                title: "Pinging other members".to_string(),
                description: "Do not ping other people in order to get attention to your question unless they are actively involved in the discussion.".to_string(),
            }),
            Some("!promotion") => Some(Command {
                name: "!promotion".to_string(),
                title: "Self promotion".to_string(),
                description: "We have a few channels that allow for self-promotion [#showcase, #community-content]. Sharing promotional links such as referral links, giveaways/contests
                or anything that would be a plain advertisment is discouraged and may be removed.".to_string(),
            }),
            Some("!jobs") => Some(Command {
                name: "!jobs".to_string(),
                title: "Job postings".to_string(),
                description: "We currently do not allow job posts in this server, unless it's in the context of a discussion. If you're looking to get hired or to advertise a job vacancy
                see #jobs.".to_string(),
            }),
            Some("!DM") => Some(Command {
                name: "!DM".to_string(),
                title: "Sending direct messages to other members".to_string(),
                description: "Do not directly message other members without asking first. If you're looking to get help, it is a lot better to post your question in the applicable channel publicly.".to_string(),
            }),
            _ => None,
        }
    }
}

fn matches_message(command: &str) -> Option<&str> {
    let content = command.trim();

    if content.starts_with(command) || command.eq(content) {
        return Some(command);
    }

    None
}

async fn send_embed(
    msg: &Message,
    ctx: &Context,
    command: Command,
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
