use args::Chdisassembler;
use clap::Parser;
mod args;

fn main() {
    let cli = Chdisassembler::parse();
    if let Err(e) = cli.dissassemble() {
        eprintln!("Error: {}", e);
    }
}