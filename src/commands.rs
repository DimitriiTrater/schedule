use teloxide::utils::command::BotCommands;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
pub enum Command {
    #[command(description = "вывести команды для бота")]
    Help,
    #[command(description = "Начать поиск расписания")]
    Start,
    #[command(description = "Сбросить специальность")]
    Reset,
    #[command(description = "Назад")]
    Back,
    #[command(description = "Отменить диалог")]
    Cancel
}


