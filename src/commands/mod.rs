use anyhow::anyhow;
use serenity::{
    model::prelude::{ChannelId, GuildId},
    prelude::Context as SerenityContext,
};

pub mod background_commands;
pub mod implicit_commands;
pub mod named_commands;

type Error = anyhow::Error;

pub fn general_channel(ctx: &SerenityContext, guild_id: GuildId) -> Result<ChannelId, Error> {
    let guild = ctx
        .cache
        .guild(guild_id)
        .ok_or(anyhow!("couldn't find guild"))?;
    let channel_id = guild
        .channels
        .iter()
        .find(|(_, channel)| match channel {
            serenity::model::prelude::Channel::Guild(chan) => chan.name == "general",
            serenity::model::prelude::Channel::Private(_) => false,
            serenity::model::prelude::Channel::Category(_) => false,
            _ => false,
        })
        .map(|(id, _)| *id)
        .ok_or(anyhow!("couldn't find general channel"))?;
    Ok(channel_id)
}
