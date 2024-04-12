use crate::schemas::aliases::{DialogueHandler, ResultHandler};
use crate::Command;
use crate::State;

use teloxide::payloads::SendMessageSetters;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn start(bot: Bot, dialogue: DialogueHandler, msg: Message) -> ResultHandler {
    bot.send_message(
        msg.chat.id,
        "Введи свою группу как ты бы её ввёл тут\n|\n -> https://schedule.kantiana.ru/"
    ).await?;
    dialogue.update(State::SetSpeciality).await?;
    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> ResultHandler {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> ResultHandler {
    bot.send_message(
        msg.chat.id,
        "Я не знаю, что ответить на это. Набери /help чтобы увидеть все команды.",
    )
    .await?;
    Ok(())
}

pub async fn set_speciality(bot: Bot, dialogue: DialogueHandler, msg: Message) -> ResultHandler {
    log::info!("set_speciality");
    match msg.text().map(ToOwned::to_owned) {
        Some(speciality) => {
            let stages = ["Получить расписание", "Сменить группу"]
                .map(|stage| InlineKeyboardButton::callback(stage, stage));
            bot.send_message(msg.chat.id, "Select:")
                .reply_markup(InlineKeyboardMarkup::new([stages]))   
                .await?;
            dialogue.update(State::ReceiveSpeciality { speciality }).await?;
        }
        None => {
            let _ = bot.send_message(msg.chat.id, "None");
        }
    }
    Ok(())
}

pub async fn receive_product_selection(
    bot: Bot,
    dialogue: DialogueHandler,
    speciality: String,
    q: CallbackQuery,
) -> ResultHandler {
    if let Some(stage) = &q.data {
        bot.send_message(dialogue.chat_id(), format!("{stage} {speciality}")).await?;
        dialogue.exit().await?;
    }
    Ok(())
}
