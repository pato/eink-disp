use eink_disp::{f1::draw_next_race, EinkDisplay};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut eink = EinkDisplay::new();
    draw_next_race(&mut eink).await?;

    eink.save_header_file("/tmp/arduino_header")?;

    Ok(())
}
