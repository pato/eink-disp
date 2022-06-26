use crate::EinkDisplay;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use ergast_rs::{
    apis::race_table::{Driver, QualifyingResult, Race},
    Ergast,
};
use eyre::{eyre, Result};
use unidecode::unidecode;

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

pub async fn draw_last_qualifying_results<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
) -> Result<()> {
    let ergast = Ergast::new()?;
    let quali = ergast.qualifying_results(None, None).await?;
    let race = quali
        .races
        .first()
        .ok_or(eyre!("No quali results fetched!"))?;

    draw_last_qualifying_results_fetched(eink, race).await
}

async fn draw_last_qualifying_results_fetched<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
    race: &Race,
) -> Result<()> {
    let quali = race
        .qualifying_results
        .as_deref()
        .ok_or(eyre!("Missing qualifying results!"))?;

    let big_positions = 3;
    let mut y = 20;

    let mut positions = String::new();
    for position in quali.iter().take(big_positions) {
        positions.push_str(&format_qualifying_position(position));
    }
    eink.draw_big_text(&positions, 0, y, false);
    y += 60; // 3 * 20 for font

    let mut positions = String::new();
    for position in quali.iter().skip(big_positions) {
        positions.push_str(&format_qualifying_position(position))
    }
    eink.draw_medium_text(&positions, 5, y, false);

    Ok(())
}

fn format_driver_name(driver: &Driver) -> String {
    // Since we can only render ASCII values, decode the unicode characters to their closest ASCII
    // counterpart.
    let first_name = unidecode(&driver.given_name);
    let last_name = unidecode(&driver.family_name);

    format!("{} {}", first_name, last_name)
}

fn format_qualifying_position(position: &QualifyingResult) -> String {
    let driver_name = format_driver_name(&position.driver);
    let time = position
        .q3
        .as_deref()
        .or(position.q2.as_deref())
        .or(position.q1.as_deref())
        .unwrap_or("N/A");
    format!(
        "{:2}. {:16} {:8}  {}\n",
        position.position, driver_name, time, &position.constructor.name,
    )
}
