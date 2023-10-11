use anyhow::Context;
use serenity::{
    async_trait,
    client::Context as SerenityContext,
    framework::StandardFramework,
    model::prelude::{Message, Ready, ResumedEvent},
    prelude::{EventHandler, GatewayIntents},
    utils::MessageBuilder,
    Client,
};
use tracing::{error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

type Error = anyhow::Error;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip_all)]
    async fn ready(&self, _ctx: SerenityContext, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    #[instrument(skip_all)]
    async fn resume(&self, _ctx: SerenityContext, _resumed: ResumedEvent) {
        info!("Resumed");
    }

    #[instrument(skip(self, ctx))]
    async fn message(&self, ctx: SerenityContext, msg: Message) {
        if msg.mentions.iter().any(|u| u.name == "RoboFerris") {
            let author = msg.author;
            info!(%author, "bot was mentioned");
            let response = MessageBuilder::new()
                .push("Beep boop to you, ")
                .mention(&author)
                .build();
            if let Err(err) = msg.channel_id.say(ctx.http, response).await {
                error!(%err, "couldn't send reply");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fancy tree-structured tracing output
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();

    // Get the token from the environment
    let discord_token = std::env::var("DISCORD_TOKEN").context("DISCORD_TOKEN not found")?;

    // Set up the bot framework and allow it to recognize commands as messages
    // starting with a few different characters.
    let framework = StandardFramework::new().configure(|c| c.prefixes(["!", "~", ">"]));

    let intents = GatewayIntents::GUILD_MESSAGES  // notified when a message is created, etc
        | GatewayIntents::GUILD_MESSAGE_REACTIONS // add reactions to messages
        | GatewayIntents::GUILD_MESSAGE_TYPING    // see when someone is typing
        | GatewayIntents::MESSAGE_CONTENT         // read the content of messages
        | GatewayIntents::GUILD_PRESENCES; // notified when someone goes idle, etc

    let mut client = Client::builder(&discord_token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .context("couldn't create Discord client")?;

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
