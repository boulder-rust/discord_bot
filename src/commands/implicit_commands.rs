//! This module dispatches on commands that don't have specific names. For example, you
//! might have a command that parses messages in the format `~foo is bar`, storing the
//! key-value pair `foo` and `bar` so that you can later recall the value by typing `~foo`.
//!
//! This is a command in the sense that it has a well-defined syntax and a specific purpose,
//! but it doesn't have a name that you use to call the command.

use serenity::{model::prelude::Message, prelude::Context as SerenityContext};

pub async fn handle_implicit_command(
    _ctx: SerenityContext,
    _msg: Message,
) -> Result<(), anyhow::Error> {
    // Call each command in sequence here.
    // my_command(ctx.clone(), msg.clone()).await;
    todo!()
}

// async fn my_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
//     todo!()
// }
