use crate::EinkDisplay;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use ergast_rs::{
    apis::race_table::{QualifyingResult, Race},
    Ergast,
};
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

const LINE: &str = "                                                                                                       ";

async fn draw_last_qualifying_results_fetched<D: DrawTarget<Color = BinaryColor>>(
    eink: &mut EinkDisplay<D>,
    race: &Race,
) -> Result<()> {
    let quali = race
        .qualifying_results
        .as_deref()
        .ok_or(eyre!("Missing qualifying results!"))?;

    let mut iter = quali.iter();
    let mut y = 50;

    let pole_position = iter.next().ok_or(eyre!("Missing pole position"))?;
    let pole = format_pole_position(pole_position);
    eink.draw_big_text(&pole, 200, y, true);
    y += 25; // 20 for font, 5 padding

    let mut positions = String::new();
    for position in iter {
        positions.push_str(&format_qualifying_position(position))
    }
    positions.push('\n');
    positions.push_str(LINE);
    eink.draw_small_text(&positions, 50, y, false);

    Ok(())
}

fn format_pole_position(position: &QualifyingResult) -> String {
    let driver_name = format!(
        "{} {}",
        &position.driver.given_name, &position.driver.family_name
    );
    let time = position
        .q3
        .as_deref()
        .or(position.q2.as_deref())
        .or(position.q1.as_deref())
        .unwrap_or("N/A");
    format!(
        "{}. {} {}  {}\n",
        position.position, driver_name, time, &position.constructor.name,
    )
}

fn format_qualifying_position(position: &QualifyingResult) -> String {
    let driver_name = format!(
        "{} {}",
        &position.driver.given_name, &position.driver.family_name
    );
    let time = position
        .q3
        .as_deref()
        .or(position.q2.as_deref())
        .or(position.q1.as_deref())
        .unwrap_or("N/A");
    format!(
        "{:2}. {:18} {:8}  {}\n",
        position.position, driver_name, time, &position.constructor.name,
    )
}
