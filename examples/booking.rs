/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::error::Error;

use talk_loco_client::{
    client::booking::BookingClient, command::session::BsonCommandSession, request,
};
use tokio::{net::TcpStream, io::BufStream};
use tokio_native_tls::native_tls;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connector = tokio_native_tls::TlsConnector::from(native_tls::TlsConnector::new().unwrap());

    let stream = connector
        .connect(
            "booking-loco.kakao.com",
            BufStream::new(
                TcpStream::connect("booking-loco.kakao.com:443")
                    .await
                    .unwrap(),
            ),
        )
        .await
        .unwrap()
        .compat();

    let mut booking_conn = BsonCommandSession::new(stream);
    let mut booking_client = BookingClient(&mut booking_conn);

    let booking_res = booking_client
        .get_conf(&request::booking::GetConfReq {
            os: "win32".into(),
            mccmnc: "999".into(),
            model: "".into(),
        })
        .await?;

    println!("GETCONF response: {:?}", booking_res);

    Ok(())
}
