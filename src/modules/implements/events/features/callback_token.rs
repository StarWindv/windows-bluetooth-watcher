use crate::modules::types::events::features::callback_token::{
    CallbackToken,
    TOKEN_COUNTER
};
use std::sync::atomic::Ordering;

impl CallbackToken {
    pub fn new() -> Self {
        let id = TOKEN_COUNTER
            .fetch_add(1, Ordering::SeqCst);
        Self { id }
    }

    pub fn __str__(&self) -> String {
        format!("CallbackToken({})", self.id)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
