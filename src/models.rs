use std::collections::HashMap;
use std::sync::Arc;
use teloxide::types::UserId;
use tokio::sync::Mutex;

pub type UserStates = Arc<Mutex<HashMap<UserId, UserState>>>;

#[derive(Clone, Default)]
// TODO: store multiple inputs to create a long prompt based on that.
pub struct UserState {
    pub platform: Option<String>,
    pub theme_description: Option<String>,
}
