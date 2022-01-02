/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::SETST;
use crate::Request;

/// Update client status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStReq {

    /// Status
    ///
    /// * Unlocked = 1
    /// * Locked = 2
    #[serde(rename = "st")]
    pub status: i8

}

impl Request for SetStReq {
    const METHOD: &'static str = SETST;
}