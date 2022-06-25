use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_6X9},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::{Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use epd_waveshare::color::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let small_text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    let big_text_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    let gap = 60_u32;
    let circle_diameter = 48_u32;
    let square_length = 34_u32;
    for i in 0..5_i32 {
        Circle::new(Point::new(67 + i * (gap as i32), 8), circle_diameter)
            .into_styled(line_style)
            .draw(&mut display)?;

        Rectangle::new(
            Point::new(74 + i * (gap as i32), 15),
            Size::new(square_length, square_length),
        )
        .into_styled(line_style)
        .draw(&mut display)?;
    }

    // Text::new("Hello World!", Point::new(5, 12), text_style).draw(&mut display)?;

    // let style = MonoTextStyleBuilder::new()
    //     .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
    //     .text_color(White)
    //     .background_color(Black)
    //     .build();

    // let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::new("Next race:", Point::new(150, 100), big_text_style).draw(&mut display)?;

    let _ =
        Text::new("British Grand Prix", Point::new(100, 125), big_text_style).draw(&mut display)?;
    let _ = Text::new(
        "Silverstone Circuit",
        Point::new(140, 150),
        small_text_style,
    )
    .draw(&mut display)?;
    let _ = Text::new("England", Point::new(170, 161), small_text_style).draw(&mut display)?;

    // let _ = Text::with_text_style("Hello rust!", Point::new(175, 250), style, text_style)
    //     .draw(&mut display);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    Window::new("E-Ink Simulator", &output_settings).show_static(&display);

    Ok(())
}
