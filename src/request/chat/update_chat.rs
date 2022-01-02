/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::UPDATECHAT;
use crate::Request;

/// Update chatroom push setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    #[serde(rename = "pushAlert")]
    pub push_alert: bool

}

impl Request for UpdateChatReq {
    const METHOD: &'static str = UPDATECHAT;
}