#[cfg(feature = "tool-call")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use noctisroll::prelude::*;
    use rig_core::{
        client::{CompletionClient, ProviderClient},
        integrations::cli_chatbot::ChatBotBuilder,
        providers::deepseek,
    };

    let _ = dotenvy::dotenv();
    let deepseek = deepseek::Client::from_env()?;

    let agent = deepseek
        .agent("deepseek-v4-flash")
        .preamble("You are a dice rolling assistant.")
        .tools(tools())
        .temperature(0.7)
        .build();

    ChatBotBuilder::new()
        .agent(agent)
        .max_turns(6)
        .build()
        .run()
        .await?;

    Ok(())
}
