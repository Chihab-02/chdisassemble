use std::path::Path;
use std::fs;
use capstone::prelude::*;
use clap::{ValueEnum,Parser};

/// chdisassemble — a tiny disassembler CLI in Rust
#[derive(Parser, Debug)]
#[command(name = "chdisassemble", author, version, about = "A simple disassembler CLI")]
pub struct Chdisassembler {
    /// Input binary file
    #[arg(short = 'f', long = "file")]
    pub input: String,

    /// Output assembly file
    #[arg(short = 'o', long = "output")]
    pub output: String,

    /// Architecture (available: x86, arm, mips, riscv)
    #[arg(short = 'a', long = "arch", default_value = "x86")]
    pub arch: Architecture,

}
#[derive(Copy,ValueEnum, Clone, Debug)]
pub enum Architecture {
    X86,
    ARM,
    MIPS,
    RISCV,
}

impl Chdisassembler{
    pub fn dissassemble(&self)->Result<(),Box<dyn std::error::Error>>{
        let code =std::fs::read(Path::new(&self.input)).expect("Failed to read input file");
        
        let cs = match self.arch{
        Architecture::X86=>  
            Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .map_err(|e| format!("Capstone init error: {}", e))?,
         
        Architecture::ARM=>
            Capstone::new()
                .arm()
                .mode(arch::arm::ArchMode::Arm)
                .build()
                .map_err(|e| format!("Capstone init error: {}", e))?,
        Architecture::MIPS=>
            Capstone::new()
                .mips()
                .mode(arch::mips::ArchMode::Mips32)
                .build()
                .map_err(|e| format!("Capstone init error: {}", e))?,
        Architecture::RISCV=>
            Capstone::new()
                .riscv()
                .mode(arch::riscv::ArchMode::RiscV64)
                .build()
                .map_err(|e| format!("Capstone init error: {}", e))?,    
        };   

        let insns = cs.disasm_all(&code, 0x1000)
            .expect("Failed to disassemble");
        let mut output = String::new();
        for i in insns.iter(){
            output.push_str(&format!(
                "0x{:x}: {}\t{}\n",
                i.address(),
                i.mnemonic().unwrap_or(""),
                i.op_str().unwrap_or("")
            ));
        }
        fs::write(Path::new(&self.output), output)?;

        println!("Disassembly written to the output file.");
        
        Ok(())
    }
}
    
