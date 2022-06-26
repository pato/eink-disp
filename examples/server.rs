use eink_disp::server::EinkServer;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = EinkServer::new(([127, 0, 0, 1], 3030));
    server.serve().await
}
