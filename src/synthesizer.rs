use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::error::Error;

use crate::prompts;

pub struct ThemeSynthesizer {}

impl ThemeSynthesizer {
    pub async fn synthesize(template: &str, description: &str) -> Result<String, Box<dyn Error>> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(prompts::system_message(template))
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(description.to_string())
                    .build()?
                    .into(),
            ])
            .build()?;

        let response = Client::new().chat().create(request).await?;
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                return Ok(content.clone());
            }
        }

        Err("No response generated".into())
    }
}
