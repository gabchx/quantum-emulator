# quantum-emulator

Homemade quantum emulator, made with Rust & Python for resources benchmark.

## Usage

![qe](https://github.com/user-attachments/assets/a4052e59-6397-4d99-b248-8d85c5daf70f)


• Drag and drop gates from the toolbox.  
• Add qubits on `Qubit +`.  
• Load exemple algorithme on `Examples ::`.  
• Export to code on `Code <>`.  
• Remove qubits or gates by double clicking.  

> [!TIP]
> On rendering bugs : **Unzoom + ctrl-R**

### Exemple using Deutsch-Jozsa Algorithm

Given an n-bit Oracle (“black box”) function, we don’t know exactly what is inside but we know it is either constant (i.e., always return 0 or 1 whenever it’s queried) or balanced (i.e., 50% return 0 and 50% return 1).  
The objective is to determine whether that oracle is constant or balanced with minimum queries.  
We need to query at least twice to solve this problem with a classical solution. If we get the same output twice, we can’t conclude and need to ask again, up to N/2 + 1 queries, where N = 2^n is the number of all realizable bit strings from n bits input of the Oracle.  
However, this problem becomes trivial to solve with a quantum solution.
For this, we use [Deutsch-Jozsa Algorithm](https://en.wikipedia.org/wiki/Deutsch–Jozsa_algorithm)

![Capture d’écran 2024-10-14 à 18 56 19](https://github.com/user-attachments/assets/cabdf562-8412-4a87-8651-f5d56d3e6b67)

As the result is 1111 (with 0% of measuring 0000), we can conclude that the given oracle is balanced as expected. If the result is 0000 (with 100% of measuring 0000), that function will be constant.

### About Bloch sphere visualisation
When qubits are entangled, their states are interconnected, so you can't represent them individually on Bloch spheres—doing so doesn't capture the whole picture. However, visualizing each qubit on a Bloch sphere can still be helpful when qubits aren't entangled, or for understanding certain parts of the system. This kind of visualization shows each qubit's state and phase, giving insights into how they're behaving, but it won’t fully represent the complex relationships between qubits in the hole entangled system.  
  
![image](https://github.com/user-attachments/assets/a6586a6a-4171-4574-bab2-1eaa71dcb152)

## Developpment

![Backend Python](https://github.com/user-attachments/assets/6d620957-b589-4a99-a7fe-e839cee0803d)

### Features to do

• Implement measurement gate.  
• Add the possibility to add a custom gate.  
• Add control over other gate.  
• Optimize Rust variable size.  
• Fixe few front bugs.  

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
