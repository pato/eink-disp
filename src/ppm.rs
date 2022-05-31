use std::io::{self, Write};

use smallvec::SmallVec;

const PPM_MAGIC: &[u8] = "P3".as_bytes();
const PPM_NEW_LINE: &[u8] = "\n".as_bytes();
const PPM_SPACE: &[u8] = " ".as_bytes();
const PPM_MAX_COLOR_VAL: &[u8] = "255".as_bytes();
const PPM_MAX_CHARS_PER_LINE: usize = 70;

const ZERO_DIGIT_BYTE: u8 = 48;
const SPACE_BYTE: u8 = 32;

fn write_ppm_header(width: usize, height: usize, writer: &mut impl Write) -> io::Result<usize> {
    let mut written = 0;

    written += writer.write(PPM_MAGIC)?;
    written += writer.write(PPM_NEW_LINE)?;
    written += writer.write(&small_u32_as_utf8(width as u32))?;
    written += writer.write(PPM_SPACE)?;
    written += writer.write(&small_u32_as_utf8(height as u32))?;
    written += writer.write(PPM_NEW_LINE)?;
    written += writer.write(PPM_MAX_COLOR_VAL)?;
    written += writer.write(PPM_NEW_LINE)?;

    Ok(written)
}

pub fn write_ppm_buffer(
    width: usize,
    height: usize,
    buf: &[u8],
    writer: &mut impl Write,
) -> io::Result<()> {
    write_ppm_header(width, height, writer)?;

    let mut written_in_line = 0;
    for (idx, c) in buf.iter().enumerate() {
        let encoded_c = pixel_as_utf8(*c);

        // note: `+ 1` because we need to ensure we'd be able to fit the `\n`
        let pixel_fits_in_line = written_in_line + encoded_c.len() + 1 > PPM_MAX_CHARS_PER_LINE;
        let need_new_row = idx > 0 && idx % width == 0;
        if pixel_fits_in_line || need_new_row {
            writer.write_all(PPM_NEW_LINE)?;
            written_in_line = 0;
        } else if idx > 0 {
            written_in_line += writer.write(PPM_SPACE)?;
        }
        written_in_line += writer.write(&encoded_c)?;
    }

    writer.write_all(PPM_NEW_LINE)?;
    writer.write_all(PPM_NEW_LINE)?;

    Ok(())
}

#[inline(always)]
pub fn small_u32_as_utf8(val: u32) -> SmallVec<[u8; 8]> {
    debug_assert!(val < 100000);

    let mut vec = SmallVec::with_capacity(8);
    let mut val = val;
    for _ in 0..6 {
        vec.insert(0, (val % 10) as u8 + ZERO_DIGIT_BYTE);
        val /= 10;
        if val == 0 {
            break;
        }
    }

    vec
}

#[inline(always)]
pub fn pixel_as_utf8(pixel: u8) -> SmallVec<[u8; 16]> {
    let r = pixel as u32;
    let r = 255 - r; // flip it because the e-reader has 0xFF as black
    let g = r;
    let b = r;

    let mut vec = SmallVec::with_capacity(16);

    // encode b
    let mut val = b;
    for _ in 0..3 {
        vec.insert(0, (val % 10) as u8 + ZERO_DIGIT_BYTE);
        val /= 10;
        if val == 0 {
            break;
        }
    }

    // space
    vec.insert(0, SPACE_BYTE);

    // encode g
    let mut val = g;
    for _ in 0..3 {
        vec.insert(0, (val % 10) as u8 + ZERO_DIGIT_BYTE);
        val /= 10;
        if val == 0 {
            break;
        }
    }

    // space
    vec.insert(0, SPACE_BYTE);

    // encode r
    let mut val = r;
    for _ in 0..3 {
        vec.insert(0, (val % 10) as u8 + ZERO_DIGIT_BYTE);
        val /= 10;
        if val == 0 {
            break;
        }
    }

    vec
}
