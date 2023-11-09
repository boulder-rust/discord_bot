use anyhow::Context;
use serenity::{
    async_trait,
    client::Context as SerenityContext,
    framework::StandardFramework,
    model::prelude::{Member, Message, Ready, ResumedEvent},
    prelude::{EventHandler, GatewayIntents, TypeMapKey},
    Client,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

mod commands;

type Error = anyhow::Error;

/// A dummy struct that will be used to access the global database connection pool
struct DbConnection;
impl TypeMapKey for DbConnection {
    type Value = Pool<Postgres>;
}

struct Handler;

// If you want to respond to specific events, fill out an implementation for one of the methods on
// serenity::EventHandler here.
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

    #[instrument(
        skip_all,
        fields(
            msg.id = msg.id.0,
            msg.channel.id = msg.channel_id.0,
            msg.author = msg.author.name
        ))]
    async fn message(&self, ctx: SerenityContext, msg: Message) {
        if msg.content.starts_with('!') {
            debug!("dispatching a named command");
            if let Err(err) =
                commands::named_commands::handle_named_command(ctx.clone(), msg.clone()).await
            {
                error!(%err, "couldn't execute named command");
            }
        } else if msg.content.starts_with('~') {
            debug!("dispatching an implicit command");
            if let Err(err) =
                commands::implicit_commands::handle_implicit_command(ctx.clone(), msg.clone()).await
            {
                error!(%err, "couldn't execute implicit command");
            }
        } else {
            debug!("responding to generic message event");
            if let Err(err) = commands::background_commands::handle_message_event(ctx, msg).await {
                error!(%err, "couldn't response to message event");
            }
        }
    }

    async fn guild_member_addition(&self, ctx: SerenityContext, member: Member) {
        todo!()
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

    // Connect to the database.
    // Note that PgConnectOptions reads values from the environment even though
    // we don't pass it any arguments here.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(PgConnectOptions::new())
        .await?;

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
        .type_map_insert::<DbConnection>(pool) // this is where we add the database connection
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
