use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*
};

use crate::State;

pub type DialogueHandler = Dialogue<State, InMemStorage<State>>;
pub type ResultHandler = Result<(), Box<dyn std::error::Error + Send + Sync>>;
