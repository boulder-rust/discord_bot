//! This module dispatches on named commands, e.g. `!mycmd arg1 arg2`.

use super::Error;
use serenity::{model::prelude::Message, prelude::Context as SerenityContext};

pub async fn handle_named_command(_ctx: SerenityContext, _msg: Message) -> Result<(), Error> {
    // Call each command in sequence here.
    // my_command(ctx.clone(), msg.clone()).await?;
    todo!()
}

// async fn my_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
//     todo!()
// }
