use crate::UserState;
use std::error::Error;

// TODO: setup prompts

pub struct Synthesizer {
    state: UserState,
}

impl Synthesizer {
    pub fn new(state: UserState) -> Self {
        Self { state }
    }

    pub async fn synthesize(&self) -> Result<String, Box<dyn Error>> {
        // TODO: call gpt with the state.description and base prompt for selected platform
        // simply match {state.platform}
        Ok(format!(
            "Synthesizing theme for {} based on: {}",
            self.state.platform.as_ref().unwrap(),
            self.state.theme_description.as_ref().unwrap()
        ))
    }
}
