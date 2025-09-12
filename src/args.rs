use capstone::prelude::*;
use clap::{Parser, ValueEnum};
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;
use std::path::Path;

/// chdisassemble — a tiny disassembler CLI in Rust
#[derive(Parser, Debug)]
#[command(
    name = "chdisassemble",
    author,
    version,
    about = "A simple disassembler CLI"
)]
pub struct Chdisassembler {
    /// Input binary file
    #[arg(short = 'f', long = "file")]
    pub input: String,
    /// Output assembly file
    #[arg(short = 'o', long = "output")]
    pub output: String,
    /// Architecture (available: x86, arm (not thumb), mips, riscv)
    #[arg(short = 'a', long = "arch", default_value = "x86")]
    pub arch: Architecture,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Architecture {
    X86,
    ARM,
    MIPS,
    RISCV,
}

impl Chdisassembler {
    pub fn disassemble(&self) -> Result<(), Box<dyn Error>> {
        let data = fs::read(&self.input)?;
        let output_path = Path::new(&self.output);
        
        // Try parsing as object file
        if let Ok(file) = object::File::parse(&*data) {
            let cs = self.make_capstone(self.arch)?;
            let mut output = String::new();
            
            for section in file.sections() {
                let name = section.name().unwrap_or("<unnamed>").to_string();
                
                // Only process .text section for now (removed undefined all_sections field)
                if name != ".text" {
                    continue;
                }
               
                let data = match section.data() {
                    Ok(d) if !d.is_empty() => d,
                    _ => continue,
                };
                
                println!("Disassembling section: {}", name);
                output.push_str(&format!("Disassembly of section .text:\n"));
                output.push_str(&format!("  Size: {}\n", section.size()));
                output.push_str(&format!("  Address: {:#x}\n\n", section.address()));
                let insns = cs.disasm_all(data, section.address()).expect("Failed to disassemble section");
                
                for i in insns.iter() {
                    output.push_str(&format!(
                        "{:08x}:\t{:<8}\t{}\n",
                        i.address(),
                        i.mnemonic().unwrap_or(""),
                        i.op_str().unwrap_or("")
                    ));
                }
            }
            fs::write(output_path, output)?;
        } else {
            // treat it as a flat binary blob
            let cs = self.make_capstone(self.arch)?;
            let insns = cs.disasm_all(&data, 0x1000).expect("Failed to disassemble binary");
            let mut output = String::new();
            
            for i in insns.iter() {
                output.push_str(&format!(
                    "{:08x}:\t{:<8}\t{}\n",
                    i.address(),  
                    i.mnemonic().unwrap_or(""),
                    i.op_str().unwrap_or("")
                ));
            }
            fs::write(output_path, output)?;
        }
        
        println!("Disassembly written to {}", output_path.display());
        Ok(())
    }
    
    fn make_capstone(&self, arch: Architecture) -> Result<Capstone, Box<dyn Error>> {
        let cs = match arch {
            Architecture::X86 => Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()?,
            Architecture::ARM => Capstone::new()
                .arm()
                .mode(arch::arm::ArchMode::Arm)
                .build()?,
            Architecture::MIPS => Capstone::new()
                .mips()
                .mode(arch::mips::ArchMode::Mips32)
                .build()?,
            Architecture::RISCV => Capstone::new()
                .riscv()
                .mode(arch::riscv::ArchMode::RiscV64)
                .build()?,
        };
        Ok(cs)
    }
}