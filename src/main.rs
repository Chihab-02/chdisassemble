use clap::Parser;
mod args;

use args::Chdisassembler;
fn main() {
    let cli = Chdisassembler::parse();
    if let Err(e) = cli.dissassemble() {
        eprintln!("Error: {}", e);
    }
}