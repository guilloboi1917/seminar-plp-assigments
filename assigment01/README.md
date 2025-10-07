# Rust Assigment 01 Project
Noah Isaak - 13-929-476

## Building and Running
Cargo (Rust package manager) was used to create and manage this assignment's project.

To compile all binaries and libraries:
```bash
cargo build
```

After compiling, binaries can be found in:
```bash
target/debug/
```

The project contains multiple binaries (encode-decode, fizzbuzz, hello-world, snippets) with can be run as follows with cargo:
```bash
cargo run --bin <binary_name>
```

e.g. 
```bash
cargo run --bin encode-decode
```

## Structure
- /compiled_binaries contains already compiled binaries for linux
- /target folder was omitted to minimize hand-in file size
- /src/bin/ contains the executable binaries after building the project
- lib.rs contains shared code to be used by other binaries
- Files in /src/ are modules to be used in other binaries (e.g. fn quicksort used in snippets)

## Installing Rust

### **Windows**
```bash
winget install Rustlang.Rustup
# or use the official installer:
# https://www.rust-lang.org/tools/install
```

### **Windows**
```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

### **macOS**
```bash
brew install rustup
rustup-init
source $HOME/.cargo/env
```

**Verify Installation**
```bash
rustc --version
cargo --version
```
