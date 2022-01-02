/*
 * Created on Sun Dec 12 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

use crate::{request, RequestAndResponse};
use crate::method::FORWARD;
use crate::response::chat::ForwardRes;

/// Forward chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardReq {
    /// [request::chat::Write] content to forward
    #[serde(flatten)]
    pub content: request::chat::WriteReq
}

impl RequestAndResponse for ForwardReq {
    const METHOD: &'static str = FORWARD;
    type Response = ForwardRes;
}