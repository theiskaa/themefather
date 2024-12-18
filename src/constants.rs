//! Constants module for the Theme Father Bot
//!
//! This module contains all the constant string messages used throughout the bot,
//! including welcome messages, prompts for different platforms, and other static text.

/// Welcome message shown when user starts the bot, listing all available commands
pub const WELCOME_MESSAGE: &str = "Welcome to Theme Father Bot! 🎨\n\
    I can help you create Telegram themes for different platforms.\n\n\
    Available commands:\n\
    /createIosTheme     - Create theme for iOS\n\
    /createAndroidTheme - Create theme for Android\n\
    /createMacosTheme   - Create theme for macOS\n\
    /createWindowsTheme - Create theme for Windows\n\
    /reset              - Reset current theme creation process";

/// Message shown when user resets their theme creation process
pub const RESET_MESSAGE: &str = "Theme creation process has been reset.\n\n\
    Available commands:\n\
    /createIosTheme     - Create theme for iOS\n\
    /createAndroidTheme - Create theme for Android\n\
    /createMacosTheme   - Create theme for macOS\n\
    /createWindowsTheme - Create theme for Windows\n\
    /reset              - Reset current theme creation process";

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
