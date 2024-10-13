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

> Using docker (recommended)

```bash
 docker-compose up --build
```

`http://127.0.0.1:8000/home`

> Using rust and python separatly on your machine (for developpment)

```bash
# Rust backend + expose frontend
cd quantum_emulator
cargo install --path .
cargo run
```

```bash
# Python backend
cd quantum_emulator
poetry install
poetry run app
```

### To install docker compose

https://docs.docker.com/compose/install/
tips: use Docker Desktop for Windows and Mac

### To install rust

prefer install [rust-up](https://rust-lang.github.io/rustup/index.html)  
linux/mac : `curl -sSl '=https' https://sh.rustup.rs | sh`  
tips: make sure to add the `rust-analyzer` extension to your VSCode

### To install poetry

https://python-poetry.org/docs/#installing-with-the-official-installer
