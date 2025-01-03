use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{self, copy, BufReader, BufWriter, Write};
use std::path::Path;

fn compress_file(input: &Path, output: &Path) -> io::Result<()> {
    let input_file = File::open(input)?;
    let output_file = File::create(output)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(reader, Compression::default());
    copy(&mut encoder, &mut writer)?;
    writer.flush()?;
    Ok(())
}

fn decompress_file(input: &Path, output: &Path) -> io::Result<()> {
    let input_file = File::open(input)?;
    let output_file = File::create(output)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut decoder = GzDecoder::new(reader);
    copy(&mut decoder, &mut writer)?;
    writer.flush()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <compress|decompress> <input> <output>", args[0]);
        return;
    }

    let command = &args[1];
    let input = Path::new(&args[2]);
    let output = Path::new(&args[3]);

    match command.as_str() {
        "compress" => {
            if let Err(e) = compress_file(input, output) {
                eprintln!("Failed to compress file: {}", e);
            }
        }
        "decompress" => {
            if let Err(e) = decompress_file(input, output) {
                eprintln!("Failed to decompress file: {}", e);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
