use clap::Parser;
use std::error::Error;

mod args;
use args::Chdisassembler;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments using clap
    let args = Chdisassembler::parse();
    
    // Run the disassembler
    args.disassemble()?;
    
    Ok(())
}