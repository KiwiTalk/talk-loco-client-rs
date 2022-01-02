#![feature(min_specialization)]
/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Official client/server compatible Loco client and commands implementation.
//! See [request], [response] module for command datas.
//! See [structs] module for types used in command datas.
//! See [client] module for client implementation.

use serde::de::DeserializeOwned;
use serde::Serialize;
use loco_protocol::{Error, LocoInstance, LocoInstanceRead, LocoInstanceWrite};
use async_trait::async_trait;

pub mod request;
pub mod response;

pub mod structs;

pub mod stream;

pub mod method;

#[async_trait]
pub trait Send<REQ, RES> {
	type Response;
	async fn send(&self, request: &REQ) -> Result<RES, Error>;
}

#[async_trait]
pub trait SendOnly<REQ> {
	async fn send(&self, request: &REQ) -> Result<(), Error>;
}

pub trait RequestAndResponse {
	const METHOD: &'static str;
	type Response;
}

pub trait Request {
	const METHOD: &'static str;
}

#[async_trait]
impl<T: RequestAndResponse<Response=U> + Serialize + Sync, U: DeserializeOwned, R: LocoInstanceRead, W: LocoInstanceWrite> Send<T, U> for LocoInstance<R, W> {
	type Response = U;

	async fn send(&self, request: &T) -> Result<U, Error> {
		Ok(bson::from_document(self.send_and_receive_bson(T::METHOD, request).await?)?)
	}
}

#[async_trait]
impl<T: Request + Serialize + Sync, R: LocoInstanceRead, W: LocoInstanceWrite> SendOnly<T> for LocoInstance<R, W> {
	async fn send(&self, request: &T) -> Result<(), Error> {
		self.send_bson(T::METHOD, request).await
	}
}