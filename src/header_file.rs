use std::io::{self, Write};

const START_LINE: &[u8] = "const unsigned char gImage_alberteinstein[15000] = {\n".as_bytes();
const END_LINE: &[u8] = "};\n".as_bytes();

const NEW_LINE: &[u8] = "\n".as_bytes();
const COMMA: &[u8] = ",".as_bytes();

pub fn write_header_buffer(buf: &[u8], writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(START_LINE)?;

    for (idx, c) in buf.iter().enumerate() {
        writer.write_all(format!("{:#04x}", c).as_bytes())?;
        writer.write_all(COMMA)?;

        if idx > 0 && idx % 16 == 0 {
            writer.write_all(NEW_LINE)?;
        }
    }

    writer.write_all(END_LINE)?;
    Ok(())
}
