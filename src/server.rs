use std::{convert::Infallible, net::SocketAddr};

use eyre::Result;
use warp::Filter;

use crate::{f1::draw_next_race, EinkDisplay};

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
        let next_race = warp::path!("nextrace").and_then(Self::next_race);

        println!("EinkServer about listen on: {:?}", self.addr);
        let route = warp::get().and(next_race);
        warp::serve(route).run(self.addr).await;

        Ok(())
    }

    async fn next_race() -> Result<impl warp::Reply, Infallible> {
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

        Ok(definition)
    }
}
