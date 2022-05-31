use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyleBuilder},
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::{
    color::*,
    epd4in2::{Display4in2, Epd4in2},
    graphics::DisplayRotation,
    prelude::*,
};
use ppm::write_buffer;

mod ppm;

/// Based on the GDEW042T2 e-Paper from Dalian Good Display Co., Ltd.: www.good-display.com
#[derive(Default)]
pub struct EinkDisplay {
    disp: Display4in2,
}

impl EinkDisplay {
    pub fn new() -> Self {
        let mut disp = Display4in2::default();
        disp.set_rotation(DisplayRotation::Rotate0);
        disp.clear_buffer(Color::White);
        Self { disp }
    }

    pub fn as_json_arr() -> String {
        format!("todo")
    }

    pub fn as_ppm() -> String {
        todo!()
    }

    pub fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        // create the output file
        let file = File::create(file_name)?;
        // let's avoid a syscall per write and buffer our writes
        let buffer_size = 1 * 1024 * 1024; // 1 MB at a time
        let mut writer = BufWriter::with_capacity(buffer_size, file);

        write_buffer(141, 106, self.disp.buffer(), &mut writer)?;
        writer.flush()?;

        Ok(())
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
            .text_color(White)
            .background_color(Black)
            .build();

        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        let _ =
            Text::with_text_style(text, Point::new(x, y), style, text_style).draw(&mut self.disp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut display = EinkDisplay::default();
        display.draw_text("Hello rust!", 175, 250);

        display
            .save_to_file("/tmp/eink.ppm")
            .expect("failed to render");
    }
}
