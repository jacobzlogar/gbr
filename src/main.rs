use clap::Parser;
use gbr::system::System;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), args.file);
    let binary = std::fs::read(&path).expect(&format!("Couldn't find {} at {path}", args.file));
    let mut emulator = System::new(binary)?;
    emulator.execute();
    Ok(())
}
