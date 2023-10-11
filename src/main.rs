use anyhow::Context;
use serenity::{
    async_trait,
    client::Context as SerenityContext,
    framework::StandardFramework,
    model::prelude::{Ready, ResumedEvent},
    prelude::{EventHandler, GatewayIntents},
    Client,
};
use tracing::{error, info, instrument};

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
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

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
