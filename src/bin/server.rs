use eink_disp::server::EinkServer;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = ([0, 0, 0, 0], 3137);
    let mut server = EinkServer::new(addr);
    server.serve().await
}
