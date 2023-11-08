//! This module contains functions that respond to message content without necessarily
//! being a command. For example, you might have the bot send a message whenever someone
//! sends a message that contains a certain phrase.

use super::Error;
use serenity::{
    model::prelude::Message, prelude::Context as SerenityContext, utils::MessageBuilder,
};
use tracing::{error, info};

pub async fn handle_message_event(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    bot_was_mentioned(ctx, msg).await?;
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
