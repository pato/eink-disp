use crate::EinkDisplay;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use eyre::Result;

pub async fn draw_next_race<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
) -> Result<()> {
    // Draw the start lights
    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let gap = 60_u32;
    let circle_diameter = 48_u32;
    let square_length = 34_u32;
    for i in 0..5_i32 {
        let _ = Circle::new(Point::new(67 + i * (gap as i32), 48), circle_diameter)
            .into_styled(line_style)
            .draw(eink.raw_display());

        let _ = Rectangle::new(
            Point::new(74 + i * (gap as i32), 55),
            Size::new(square_length, square_length),
        )
        .into_styled(line_style)
        .draw(eink.raw_display());
    }

    // 100 character long line of whitespace
    let line = "                                                                                                       ";

    eink.draw_big_text(
        &format!("Next race:\n British Grand Prix\n{line}"),
        200,
        140,
        true,
    );
    eink.draw_small_text(
        &format!("Silverstone Circuit\nEngland\n{line}"),
        200,
        190,
        true,
    );

    Ok(())
}
