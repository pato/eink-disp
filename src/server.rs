use core::fmt;
use std::{convert::Infallible, net::SocketAddr};

use bytes::Bytes;
use eyre::Result;
use hyper::{http, Body, Response, StatusCode};
use warp::Filter;

use crate::{
    f1::{draw_last_qualifying_results, draw_next_race},
    EinkDisplay,
};

/// Represents an HTTP server which can be started and will render various screens for an eink
/// display
pub struct EinkServer {
    addr: SocketAddr,
}

impl EinkServer {
    pub fn new(addr: impl Into<SocketAddr>) -> Self {
        let addr = addr.into();
        Self { addr }
    }

    pub async fn serve(&mut self) -> Result<()> {
        let next_race = warp::path!("next_race").and_then(Self::next_race);
        let next_race_header = warp::path!("next_race_header").and_then(Self::next_race_header);
        let quali_results = warp::path!("quali_results").and_then(Self::quali_results);

        println!("EinkServer about listen on: {:?}", self.addr);
        let route = warp::get().and(next_race.or(quali_results).or(next_race_header));
        warp::serve(route).run(self.addr).await;

        Ok(())
    }

    async fn next_race_header() -> Result<impl warp::Reply, Infallible> {
        let mut eink = EinkDisplay::new();
        if let Err(err) = draw_next_race(&mut eink).await {
            return Ok(format!("Failed to fetch next race! Err={:?}", err));
        };

        let definition = match eink.header_definition() {
            Ok(it) => it,
            Err(err) => return Ok(format!("Failed to encode header definition! Err={:?}", err)),
        };

        let definition = match String::from_utf8(definition) {
            Ok(it) => it,
            Err(err) => return Ok(format!("Invalid UTF8 in header definition! Err={:?}", err)),
        };

        println!("Fetched next_race_header successfully");
        Ok(definition)
    }

    async fn next_race() -> Result<impl warp::Reply, Infallible> {
        let mut eink = EinkDisplay::new();
        if let Err(err) = draw_next_race(&mut eink).await {
            return internal_error("Failed to fetch next race!", err);
        };

        let raw_bytes = Bytes::copy_from_slice(eink.raw_bytes());
        let resp = Response::builder().body(raw_bytes.into()).unwrap();
        // let definition = match eink.header_definition() {
        //     Ok(it) => it,
        //     Err(err) => return Ok(format!("Failed to encode header definition! Err={:?}", err)),
        // };

        // let definition = match String::from_utf8(definition) {
        //     Ok(it) => it,
        //     Err(err) => return Ok(format!("Invalid UTF8 in header definition! Err={:?}", err)),
        // };

        println!("Fetched next_race successfully");
        Ok(resp)
    }

    async fn quali_results() -> Result<http::Response<Body>, Infallible> {
        let mut eink = EinkDisplay::new();
        if let Err(err) = draw_last_qualifying_results(&mut eink).await {
            return internal_error("Failed to fetch next race!", err);
        };

        let raw_bytes = Bytes::copy_from_slice(eink.raw_bytes());
        let resp = Response::builder().body(raw_bytes.into()).unwrap();

        println!("Fetched quali_results successfully");
        Ok(resp)
    }
}

fn internal_error(msg: &str, err: impl fmt::Debug) -> Result<http::Response<Body>, Infallible> {
    let text = format!("{} Err={:?}", msg, err);
    let body = Body::from(text);
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(body)
        .unwrap())
}
