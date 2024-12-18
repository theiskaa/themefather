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
            bot.send_message(
                msg.chat.id,
                "Welcome to Theme Father Bot! 🎨\n\
                I can help you create Telegram themes for different platforms.\n\n\
                Available commands:\n\
                /createIosTheme - Create theme for iOS\n\
                /createAndroidTheme - Create theme for Android\n\
                /createMacosTheme - Create theme for macOS\n\
                /createWindowsTheme - Create theme for Windows\n\
                /reset - Reset current theme creation process",
            )
            .await?;
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
            bot.send_message(
                msg.chat.id,
                "Starting drawing the theme for iOS! Please describe how you want your theme to look:"
            ).await?;
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
            bot.send_message(
                msg.chat.id,
                "Starting drawing the theme for Android! Please describe how you want your theme to look:"
            ).await?;
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
            bot.send_message(
                msg.chat.id,
                "Starting drawing the theme for macOS! Please describe how you want your theme to look:"
            ).await?;
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
            bot.send_message(
                msg.chat.id,
                "Starting drawing the theme for Windows! Please describe how you want your theme to look:"
            ).await?;
        }
        Command::Reset => {
            user_states.lock().await.remove(&user_id);
            bot.send_message(
                msg.chat.id,
                "Theme creation process has been reset.\n\n\
                Available commands:\n\
                /createIosTheme - Create theme for iOS\n\
                /createAndroidTheme - Create theme for Android\n\
                /createMacosTheme - Create theme for macOS\n\
                /createWindowsTheme - Create theme for Windows\n\
                /reset - Reset current theme creation process",
            )
            .await?;
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
                bot.send_message(msg.chat.id, "TODO: implement theme creation logic")
                    .await?;
            }
        }
    }
    Ok(())
}
