//! Prompts module for the Theme Father Bot
//!
//! This module contains constant string messages used by the bot for user interactions,
//! including welcome messages, platform-specific theme creation prompts, and command help text.
//! These messages provide a consistent user experience across the bot's interface.

/// Welcome message shown when user starts the bot, listing all available commands
pub const WELCOME_MESSAGE: &str = "Welcome to Theme Father Bot! 🎨\n\
    I can help you create Telegram themes for different platforms.\n\n\
    Available commands:\n\
    /createIosTheme - Create theme for iOS\n\
    /createAndroidTheme - Create theme for Android\n\
    /createMacosTheme - Create theme for macOS\n\
    /createWindowsTheme - Create theme for Windows\n\
    /reset - Reset current theme creation process";

/// Message shown when user resets their theme creation process
pub const RESET_MESSAGE: &str = "Theme creation process has been reset.\n\n\
    Available commands:\n\
    /createIosTheme - Create theme for iOS\n\
    /createAndroidTheme - Create theme for Android\n\
    /createMacosTheme - Create theme for macOS\n\
    /createWindowsTheme - Create theme for Windows\n\
    /reset - Reset current theme creation process";

/// Prompt shown when user starts iOS theme creation
pub const IOS_PROMPT: &str =
    "Starting drawing the theme for iOS! Please describe how you want your theme to look:";

/// Prompt shown when user starts Android theme creation
pub const ANDROID_PROMPT: &str =
    "Starting drawing the theme for Android! Please describe how you want your theme to look:";

/// Prompt shown when user starts macOS theme creation
pub const MACOS_PROMPT: &str =
    "Starting drawing the theme for macOS! Please describe how you want your theme to look:";

/// Prompt shown when user starts Windows theme creation
pub const WINDOWS_PROMPT: &str =
    "Starting drawing the theme for Windows! Please describe how you want your theme to look:";

/// Generates a system message for the theme synthesizer that instructs the AI to strictly follow
/// the provided theme template format while only modifying values after colons, preserving structure
pub fn system_message(template: &str) -> String {
    format!(
        r#"You are a specialized theme generator that strictly follows templates.
    Your sole purpose is to generate theme configurations by modifying values while preserving the exact structure and format of the template.

    STRICT RULES:
    1. Output MUST be EXACTLY in the same format as the template below
    2. Every line MUST follow the pattern "key: value"
    3. Each key-value pair MUST be on its own line
    4. Never concatenate or combine values
    5. Every color value MUST be a complete hex code (e.g., #FF5500)
    6. Preserve ALL whitespace and indentation exactly as shown
    7. Do not add ANY explanatory text or comments
    8. Do not add or remove ANY lines from the template

    Here is the exact template to follow. Replace ONLY the values after each colon:
    ```
    {}
    ```

    IMPORTANT: Your entire response must be an exact copy of this template with only the values changed. Nothing more, nothing less."#,
        template
    )
}
