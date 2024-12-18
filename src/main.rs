//! Theme Father Bot - A Telegram bot for creating themes across different platforms
//!
//! This bot allows users to create custom themes for various platforms including iOS,
//! Android, macOS and Windows. Users can describe their desired theme in natural language
//! and the bot will process their request.

mod responses;

use responses::*;
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::Mutex;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "camelCase", description = "Available commands:")]
enum Command {
    #[command(description = "Start the bot and show available commands")]
    Start,
    #[command(description = "Create a theme for iOS")]
    CreateIosTheme,
    #[command(description = "Create a theme for Android")]
    CreateAndroidTheme,
    #[command(description = "Create a theme for macOS")]
    CreateMacosTheme,
    #[command(description = "Create a theme for Windows")]
    CreateWindowsTheme,
    #[command(description = "Reset the current theme creation process")]
    Reset,
}

#[derive(Clone, Default)]
struct UserState {
    platform: Option<String>,
    theme_description: Option<String>,
}

type UserStates = Arc<Mutex<HashMap<UserId, UserState>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Theme Father Bot");

    let bot = Bot::from_env();
    let user_states: Arc<Mutex<HashMap<UserId, UserState>>> = Arc::new(Mutex::new(HashMap::new()));

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command),
        )
        .branch(dptree::endpoint(handle_message));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![user_states])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    user_states: UserStates,
) -> ResponseResult<()> {
    let user_id = msg.from.unwrap().id;

    match cmd {
        Command::Start => {
            user_states.lock().await.remove(&user_id);
            bot.send_message(msg.chat.id, WELCOME_MESSAGE).await?;
        }
        Command::CreateIosTheme => {
            let mut states = user_states.lock().await;
            states.insert(
                user_id,
                UserState {
                    platform: Some("iOS".to_string()),
                    theme_description: None,
                },
            );
            bot.send_message(msg.chat.id, IOS_PROMPT).await?;
        }
        Command::CreateAndroidTheme => {
            let mut states = user_states.lock().await;
            states.insert(
                user_id,
                UserState {
                    platform: Some("Android".to_string()),
                    theme_description: None,
                },
            );
            bot.send_message(msg.chat.id, ANDROID_PROMPT).await?;
        }
        Command::CreateMacosTheme => {
            let mut states = user_states.lock().await;
            states.insert(
                user_id,
                UserState {
                    platform: Some("macOS".to_string()),
                    theme_description: None,
                },
            );
            bot.send_message(msg.chat.id, MACOS_PROMPT).await?;
        }
        Command::CreateWindowsTheme => {
            let mut states = user_states.lock().await;
            states.insert(
                user_id,
                UserState {
                    platform: Some("Windows".to_string()),
                    theme_description: None,
                },
            );
            bot.send_message(msg.chat.id, WINDOWS_PROMPT).await?;
        }
        Command::Reset => {
            user_states.lock().await.remove(&user_id);
            bot.send_message(msg.chat.id, RESET_MESSAGE).await?;
        }
    }
    Ok(())
}

async fn handle_message(bot: Bot, msg: Message, user_states: UserStates) -> ResponseResult<()> {
    if let Some(text) = msg.clone().text() {
        let user_id = msg.from.unwrap().id;
        let mut states = user_states.lock().await;

        if let Some(state) = states.get_mut(&user_id) {
            if state.theme_description.is_none() {
                state.theme_description = Some(text.to_string());
                bot.send_message(
                    msg.chat.id,
                    format!("Got your description! I'm now creating a {} theme based on: \"{}\"\n\nProcessing...",
                        state.platform.as_ref().unwrap(),
                        text)
                ).await?;
            }
        }
    }
    Ok(())
}
