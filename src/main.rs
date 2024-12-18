use teloxide::prelude::*;

// TODO: this is a example code from teloxide docs

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the Theme Father Bot");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
