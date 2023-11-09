//! This module dispatches on named commands, e.g. `!mycmd arg1 arg2`.

use super::Error;
use serenity::{model::prelude::Message, prelude::Context as SerenityContext, utils::MessageBuilder};
use tracing::error;

pub async fn handle_named_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    // Call each command in sequence here.
    help(ctx.clone(), msg.clone()).await?;
    todo!()
}

async fn help(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    if msg.content.starts_with("!help") {
        let response = MessageBuilder::new()
            .push("Pull requests welcome, help not yet implemented: github.com/boulder-rust/discord_bot. Have fun!")
            .build();
        if let Err(err) = msg.channel_id.say(ctx.http, response).await {
            error!(%err, "couldn't send reply");
        }
    }
    Ok(())
}

// async fn my_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
//     todo!()
// }
