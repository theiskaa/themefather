//! Models for managing user state and theme creation
//!
//! This module contains the core data structures used to track user state
//! during theme creation conversations with the bot. It manages which platform
//! a user is creating a theme for and their theme description input.
//!

#![allow(dead_code)]

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

#[derive(serde::Deserialize)]
pub struct SynthesizerOutput {
    pub theme: String,
    pub description: String,
}
