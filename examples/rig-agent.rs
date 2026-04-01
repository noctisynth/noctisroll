#[cfg(feature = "tool-call")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use noctisroll::prelude::*;
    use rig::{
        client::{CompletionClient, ProviderClient},
        integrations::cli_chatbot::ChatBotBuilder,
        providers::deepseek::{self, DEEPSEEK_CHAT},
    };

    let _ = dotenvy::dotenv();
    let deepseek = deepseek::Client::from_env();

    let agent = deepseek
        .agent(DEEPSEEK_CHAT)
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
