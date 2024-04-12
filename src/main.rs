pub mod commands;
pub mod schemas;
pub mod states;

use commands::Command;
use states::State;

use schemas::main_schema::main_schema;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Start bot");

    let bot = Bot::from_env();
    Dispatcher::builder(bot, main_schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
