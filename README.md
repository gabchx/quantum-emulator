# quantum-emulator

Homemade quantum emulator, made with Rust.

## Developpment

### Features to do

• Put the gate toolbox on the nav bar  
• Upgrade the CNOT gate display  
. Fix rotation gate  
• Add phase visualization

• Add the possibility to add a custom gate

### Quickstart

> Using rust on your machine

```bash
cd quantum_emulator
cargo install --path .
cargo run
# open ../index.html
```

> Using docker

```bash
 docker-compose up --build
```

`http://0.0.0.0:8088/home`

### To install rust

prefer install [rust-up](https://rust-lang.github.io/rustup/index.html)  
linux/mac : `curl -sSl '=https' https://sh.rustup.rs | sh`  
tips: make sure to add the `rust-analyzer` extension to your VSCode
