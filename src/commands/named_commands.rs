//! This module dispatches on named commands, e.g. `!mycmd arg1 arg2`.

use super::Error;
use serenity::{
    model::prelude::Message, prelude::Context as SerenityContext, utils::MessageBuilder,
};
use tracing::error;

pub async fn handle_named_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    // Call each command in sequence here.
    help(ctx.clone(), msg.clone()).await?;
    handle_aqi_command(ctx.clone(), msg.clone()).await?;
    Ok(())
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

async fn handle_aqi_command(ctx: SerenityContext, msg: Message) -> Result<(), Error> {
    let client = reqwest::Client::new();
    if let Ok(api_token) = std::env::var("AIRNOW_API_TOKEN") {
        let response = client
            .get("https://feeds.airnowapi.org/rss/realtime/33.xml")
            .query(&[("id", api_token)])
            .send()
            .await?;
        let response = response.text().await?;
        let lines = response
            .lines()
            .filter(|line| line.contains("AQI"))
            .collect::<Vec<_>>();
        let mut final_bot_response = String::from("AQI Data for Boulder/Denver:\n");
        for line in lines {
            let line = line.trim();
            let mut decoded = String::new();
            html_escape::decode_html_entities_to_string(line, &mut decoded);
            final_bot_response.push_str(&decoded[..decoded.find('<').unwrap_or(decoded.len())]);
            final_bot_response.push('\n');
        }

        if let Err(err) = msg.channel_id.say(ctx.http, final_bot_response).await {
            tracing::error!("Unable to send response to aqi request: {err}");
        }
    }

    Ok(())
}
