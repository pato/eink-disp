use std::{
    fs::File,
    io::{BufWriter, Write},
};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_6X9},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use epd_waveshare::{color::*, epd4in2::Display4in2, graphics::DisplayRotation, prelude::*};
use eyre::Result;
use header_file::write_header_buffer;

pub mod f1;
mod header_file;

/// Based on the GDEW042T2 e-Paper from Dalian Good Display Co., Ltd.: www.good-display.com
pub struct EinkDisplay<D: DrawTarget = Display4in2> {
    disp: D,
}

impl EinkDisplay<Display4in2> {
    pub fn new() -> Self {
        let mut disp = Display4in2::default();
        disp.set_rotation(DisplayRotation::Rotate0);
        disp.clear(White).unwrap();

        Self::new_with_display(disp)
    }

    pub fn save_header_file(&self, file_name: &str) -> Result<()> {
        // create the output file
        let file = File::create(file_name)?;
        // let's avoid a syscall per write and buffer our writes
        let buffer_size = 1024 * 1024; // 1 MB at a time
        let mut writer = BufWriter::with_capacity(buffer_size, file);

        write_header_buffer(self.disp.buffer(), &mut writer)?;
        writer.flush()?;

        Ok(())
    }
}

impl Default for EinkDisplay<Display4in2> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: DrawTarget<Color = BinaryColor>> EinkDisplay<D> {
    pub fn new_with_display(disp: D) -> Self {
        Self { disp }
    }

    pub fn clear(&mut self) {
        if self.disp.clear(Black).is_err() {
            panic!("failed to clear")
        }
    }

    pub fn draw_small_text(&mut self, text: &str, x: i32, y: i32, centered: bool) {
        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
        self.draw_text(text, x, y, centered, style)
    }

    pub fn draw_big_text(&mut self, text: &str, x: i32, y: i32, centered: bool) {
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        self.draw_text(text, x, y, centered, style)
    }

    pub fn raw_display(&mut self) -> &mut D {
        &mut self.disp
    }

    fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        centered: bool,
        style: MonoTextStyle<'_, BinaryColor>,
    ) {
        let point = Point::new(x, y);
        let text = if centered {
            Text::with_alignment(text, point, style, Alignment::Center)
        } else {
            Text::new(text, Point::new(x, y), style)
        };
        let _ = text.draw(&mut self.disp);
    }

    // fn draw_text(&mut self, text: &str, x: i32, y: i32) {
    //     let style = MonoTextStyleBuilder::new()
    //         .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
    //         .text_color(White)
    //         .background_color(Black)
    //         .build();

    //     let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    //     let _ =
    //         Text::with_text_style(text, Point::new(x, y), style, text_style).draw(&mut self.disp);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut display = EinkDisplay::default();
        display.clear();

        display
            .save_header_file("/tmp/eink.h")
            .expect("failed to render");
    }
}
