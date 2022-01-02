/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::MEMBER;
use crate::RequestAndResponse;
use crate::response::chat::MemberRes;

/// Request detailed members of chatroom.
/// Official client send this when clicking profile on chatroom.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// List of requesting user id list
    #[serde(rename = "memberIds")]
    pub user_ids: Vec<i64>

}

impl RequestAndResponse for MemberReq {
    const METHOD: &'static str = MEMBER;
    type Response = MemberRes;
}