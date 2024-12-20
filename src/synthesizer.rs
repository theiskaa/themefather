use crate::{
    assets::get_theme_template,
    openai_client::{Message, OpenAIModel, OpenAIService},
    SynthesizerOutput, UserState,
};
use futures_util::StreamExt;
use std::{error::Error, io::Write};
use tokio::time::{timeout, Duration};

pub struct Synthesizer {
    state: UserState,
    openai_client: OpenAIService,
}

impl Synthesizer {
    pub fn new(state: UserState) -> Self {
        let openai_client = OpenAIService::new().expect("Cannot define a new open ai service");
        Self {
            state,
            openai_client,
        }
    }

    // TODO: improve the prompt, in order to return a valid transformed theme file, with current
    // prompt it outputs some bullshit.
    pub fn get_base_prompt(&self, platform: String, description: String) -> Vec<Message> {
        let theme_template = get_theme_template(platform.to_lowercase().as_str()).unwrap();
        let content = format!(
            r#"You are tasked with creating a theme by modifying ONLY the values in this template, keeping all field names exactly the same.

            User Description: {description}

            Template Structure:
            {platform_template}
            "#,
            platform_template = theme_template,
        );

        let system_message = r#"You are an AI theme designer. Your task is to generate theme code by ONLY modifying the values in the template.

        CRITICAL REQUIREMENTS:
        1. Keep ALL field names exactly as they appear in the template
        2. Only modify the values after the colon (:)
        3. Maintain the exact same format: "field_name: value"
        4. Use proper color codes (e.g., #FFFFFF) or specified value types
        5. Do not add or remove any fields
        6. Do not add any explanations or additional text
        7. Keep the same line breaks and spacing as the template
        8. Preserve special values like 'clear', 'light', etc. where used in the template

        Example:
        If template has:
        name: <Add Name>
        color_primary: #000000

        Your output should be like:
        name: MyTheme
        color_primary: #FF5500

        Output ONLY the theme code with modified values."#;

        vec![
            Message {
                role: "system".to_string(),
                content: system_message.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: content,
            },
        ]
    }

    pub async fn synthesize(&self) -> Result<SynthesizerOutput, Box<dyn Error>> {
        println!("Starting theme synthesis...");
        println!("Platform: {}", self.state.platform.as_ref().unwrap());
        println!(
            "Description: {}",
            self.state.theme_description.as_ref().unwrap()
        );

        let messages = self.get_base_prompt(
            self.state.platform.as_ref().unwrap().clone(),
            self.state.theme_description.as_ref().unwrap().clone(),
        );

        let stream = self
            .openai_client
            .create_chat_completion(OpenAIModel::Gpt4o, messages, 0.1)
            .await?;

        let mut full_response = String::new();
        let mut last_update = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(8); // 10 second timeout for no new content

        tokio::pin!(stream);

        loop {
            match timeout(timeout_duration, stream.next()).await {
                Ok(Some(chunk_result)) => match chunk_result {
                    Ok(Some(content)) if !content.is_empty() => {
                        print!("{}", content);
                        std::io::stdout().flush().unwrap_or_default();
                        full_response.push_str(&content);
                        last_update = std::time::Instant::now();
                    }
                    Ok(None) => break,
                    Err(e) => return Err(e),
                    _ => continue,
                },
                Ok(None) => break,
                Err(_) => {
                    if !full_response.is_empty() && last_update.elapsed() > timeout_duration {
                        println!("\nStream timed out after receiving content.");
                        break;
                    }
                    if full_response.is_empty() {
                        return Err("Stream timed out without receiving any content".into());
                    }
                }
            }
        }

        return Ok(SynthesizerOutput {
            theme: full_response,
            description: self.state.theme_description.as_ref().unwrap().clone(),
        });
    }
}
