#![allow(dead_code)]

use gbr::{cartridge::{self, Cartridge, CartridgeType, RamSize}, cpu::{Cpu, R8}, errors::SystemError, memory::Memory};
use serde::Deserialize;
use std::fs::read_dir;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Cycles {
    Values(usize, usize, String)
}

#[derive(Deserialize, Debug, Clone)]
pub struct TestState {
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
    ram: Vec<Vec<u16>>,
}

#[derive(Deserialize, Debug)]
pub struct Test {
    name: String,
    pub initial: TestState,
    r#final: TestState,
    cycles: Vec<Cycles>
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}/tests/test.json", env!("CARGO_MANIFEST_DIR"));
    let test = std::fs::read(&path)?;
    let test: Vec<Test> = serde_json::from_slice(&test)?;
    let mut cpu = Cpu::default();

    for test_case in test {
        let cartridge = setup_cartridge();
        let mut memory = setup_memory(cartridge);
        for instr in test_case.initial.ram.clone() {
            memory.rom()[instr[0].clone() as usize] = instr[1].clone() as u8;
        }
        let cpu = setup_cpu(&mut cpu, test_case.initial.clone());
        for _ in 0..test_case.initial.ram.len() {
            cpu.execute(&mut memory).map_err(|e| e.to_string())?;
        }
        println!("{:?}\n{:?}", cpu, test_case.r#final);
    }
    // let tests = format!("{}/tests", env!("CARGO_MANIFEST_DIR"));
    // for file in read_dir(tests)? {
    //     if let Some(path) = file?.path().to_str() {
    //         let file = &std::fs::read_to_string(path)?;
    //         let test: Test = serde_json::from_str(file)?;
    //         println!("{test:?}");
    //     }
    // }
    Ok(())
}

fn setup_memory(cartridge: Cartridge) -> Memory {
    Memory {
        block: [0u8; 65536],
        cartridge,
        oam_accessible: true,
        vram_accessible: true,
        rom_banks: vec![],
    }
}

fn setup_cartridge() -> Cartridge {
    Cartridge {
        rom: vec![],
        cartridge_type: CartridgeType::RomOnly,
        logo: vec![],
        title: "Test".to_string(),
        cgb_flag: false,
        rom_size: 2,
        ram_size: RamSize::Zero
    }
}

fn setup_cpu(cpu: &mut Cpu, state: TestState) -> &mut Cpu {
    cpu.registers.set_r8(R8::A, state.a);
    cpu.registers.set_r8(R8::B, state.b);
    cpu.registers.set_r8(R8::C, state.c);
    cpu.registers.set_r8(R8::D, state.d);
    cpu.registers.set_r8(R8::E, state.e);
    cpu.registers.set_r8(R8::H, state.h);
    cpu.registers.set_r8(R8::L, state.l);
    cpu.registers.flags = state.f.into();
    cpu.registers.pc = state.pc;
    cpu.registers.sp = state.sp;
    cpu
}
