use std::io::{self, BufWriter, Write, Read};
use std::env;
use std::error::Error;

use png;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    if args.len() > 1 {
        std::process::exit(1);
    }

    if args.len() == 1 && args[0] == "-d" {
        let decoder = png::Decoder::new(stdin);
        let (info, mut reader) = decoder.read_info()?;

        let mut buf = vec![0u8; info.buffer_size()];
        reader.next_frame(&mut buf)?;
        stdout.write_all(&buf)?;
        return Ok(());
    }

    let stdout = BufWriter::new(stdout);
    let mut buffer = Vec::with_capacity(4096);
    stdin.read_to_end(&mut buffer)?;

    assert!(buffer.len() % 512 == 0);
    assert!(buffer.len() < 0xFFFF_FFFF);
    let width = 512u32;
    let height = buffer.len() as u32 / width;

    let mut encoder = png::Encoder::new(stdout, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&buffer)?;

    Ok(())
}
