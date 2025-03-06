#![allow(dead_code)]

use serde::Deserialize;
use std::fs::read_dir;

#[derive(Deserialize, Debug)]
struct TestState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    ram: Vec<Vec<u32>>,
}

#[derive(Deserialize, Debug)]
pub struct Run {
    name: String,
    initial: TestState,
    r#final: TestState,
    cycles: Vec<Vec<(u32, u32, String)>>,
}

#[derive(Deserialize, Debug)]
struct Test {
    state: Vec<Run>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tests = format!("{}/tests", env!("CARGO_MANIFEST_DIR"));
    for file in read_dir(tests)? {
        if let Some(path) = file?.path().to_str() {
            let file = &std::fs::read_to_string(path)?;
            let test: Test = serde_json::from_str(file)?;
            println!("{test:?}");
        }
    }
    Ok(())
}
