/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::GETMEM;
use crate::RequestAndResponse;
use crate::response::chat::GetMemRes;
use crate::structs::ids::ChannelId;

/// Request simplified member list of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMemReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: ChannelId

}

impl RequestAndResponse for GetMemReq {
    const METHOD: &'static str = GETMEM;
    type Response = GetMemRes;
}