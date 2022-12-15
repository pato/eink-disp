use eink_disp::EinkDisplay;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

    let mut eink = EinkDisplay::new_with_display(display, BinaryColor::On, BinaryColor::Off);

    // eink_disp::f1::draw_next_race(&mut eink).await?;
    // eink_disp::f1::draw_last_qualifying_results(&mut eink).await?;
    eink_disp::f1::draw_last_race_results(&mut eink).await?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
        .build();
    Window::new("E-Ink Simulator", &output_settings).show_static(eink.raw_display());

    Ok(())
}
