# Rust Assigment 01 Project
Noah Isaak - 13-929-476


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

## Building and Running
To compile all binaries and libraries:
```bash
cargo build
```

Binaries can be found in:
```bash
target/debug/
```

The project contains multiple binaries (encode-decode, fizzbuzz, hello-world, snippets) with can be run as follows:
```bash
cargo run --bin <binary_name>
```

e.g. 
```bash
cargo run --bin encode-decode
```

## Structure
- /src/bin/ contains the executable binaries
- lib.rs contains shared code to be used by other binaries
- Files in /src/ are modules to be used in other binaries (e.g. fn quicksort used in snippets)

