//! This module contains functions that respond to message content without necessarily
//! being a command. For example, you might have the bot send a message whenever someone
//! sends a message that contains a certain phrase.

use super::Error;
use serenity::{
    model::prelude::Message, prelude::Context as SerenityContext, utils::MessageBuilder,
};
use tracing::{error, info};

pub async fn handle_message_event(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    bot_was_mentioned(ctx.clone(), msg.clone()).await?;
    carter_is_cool(ctx, msg).await?;
    Ok(())
}

async fn bot_was_mentioned(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
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
    Ok(())
}

async fn carter_is_cool(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    if msg.mentions.iter().any( |u| u.name == "CarterJ" || u.name == "minichar") {
        info!("CarterJ was mentioned");
        let response = MessageBuilder::new()
        .push("Man that guy is so cool...")
        .build();
        let res = msg.channel_id.say(ctx.http, response).await;
        if let Err(err) = res {
            error!(%err, "Carter ain't that cool apparently");
        }
    }
    Ok(())
}
