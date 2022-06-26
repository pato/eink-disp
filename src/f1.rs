use crate::EinkDisplay;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use ergast_rs::{apis::race_table::Race, Ergast};
use eyre::{eyre, Result};

pub async fn draw_next_race<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
) -> Result<()> {
    let ergast = Ergast::new()?;
    let schedule = ergast.race_schedule(None).await?;
    let next_race = schedule.next_race().ok_or(eyre!("No next race!"))?;

    draw_next_race_fetched(eink, next_race).await
}

async fn draw_next_race_fetched<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
    race: &Race,
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
    let first_line = format!("Next race:\n{}\n{line}", race.name);
    let second_line = format!(
        "{}\n{}\n{line}",
        race.circuit.circuit_name, race.circuit.location.locality
    );

    eink.draw_big_text(&first_line, 200, 140, true);
    eink.draw_small_text(&second_line, 200, 190, true);

    Ok(())
}
