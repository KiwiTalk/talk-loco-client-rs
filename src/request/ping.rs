/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::PING;
use crate::RequestAndResponse;

/// Signal server to keep connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {}

impl RequestAndResponse for Ping {
	const METHOD: &'static str = PING;
	type Response = Self;
}