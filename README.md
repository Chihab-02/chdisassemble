# chdisassemble

A small Rust command-line disassembler that converts raw binary files into human-readable assembly.

---

## Features (what this repo currently implements)

- CLI interface (flags: `-f`, `-o`, `--a`,`-s` (for strings), `-h`, `-v`).
- Disassembly using the `capstone` Rust crate.
- Architecture selection: `x86` (default) ,`arm`,`MIPS` and `RISCV`.
- Output written to a `.asm` file.

---

## 1.Build you project
clone the repo:
```bash
git clone https://github.com/Chihab-02/chdisassemble.git
cd chdisassemble
```
Build the project (make sure you have Rust installed)
``` bash
cargo build --release
cargo install --path .
```

## 2.Usage 
```bash
chdisassemble -f <binary-file> -o <output-file> --arch <architecture> 
```

PRs are WELCOMED! :3




