extern crate ndarray;
extern crate num_complex;

use ndarray::{arr2, linalg::kron, Array1, Array2};
use num_complex::Complex64;
//use std::f64::consts::PI;

#[derive(Clone)]
enum QuantumGate {
    H,
    X,
    Z,
    //Rx(f64),
    //Ry(f64),
    //Rz(f64),
}

impl QuantumGate {
    fn matrix(&self) -> Array2<Complex64> {
        match self {
            QuantumGate::H => arr2(&[
                [
                    Complex64::new(1.0 / 2f64.sqrt(), 0.0),
                    Complex64::new(1.0 / 2f64.sqrt(), 0.0),
                ],
                [
                    Complex64::new(1.0 / 2f64.sqrt(), 0.0),
                    Complex64::new(-1.0 / 2f64.sqrt(), 0.0),
                ],
            ]),
            QuantumGate::X => arr2(&[
                [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            ]),
            QuantumGate::Z => arr2(&[
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
            ]),
            _ => unimplemented!(),
        }
    }
}

struct QuantumCircuit {
    num_qubits: usize,
    state: Array1<Complex64>,
    operations: Vec<Operation>,
}

impl QuantumCircuit {
    fn new(num_qubits: usize) -> Self {
        let state_size = 1 << num_qubits;
        let mut state = Array1::<Complex64>::zeros(state_size);
        state[0] = Complex64::new(1.0, 0.0);
        QuantumCircuit {
            num_qubits,
            state,
            operations: vec![],
        }
    }

    fn add_gate(&mut self, gate: QuantumGate, qubit_idx: usize) {
        self.operations
            .push(Operation::SingleQubitGate(gate, qubit_idx));
    }

    fn add_cnot(&mut self, control: usize, target: usize) {
        self.operations
            .push(Operation::CNOTGate(CNOTGate { control, target }));
    }

    fn execute(&mut self) {
        for operation in &self.operations {
            match operation {
                Operation::SingleQubitGate(gate, qubit_idx) => {
                    let gate_matrix = self.expand_single_qubit_gate(gate, *qubit_idx);
                    self.state = gate_matrix.dot(&self.state);
                }
                Operation::CNOTGate(cnot_gate) => {
                    let cnot_matrix =
                        self.construct_cnot_matrix(cnot_gate.control, cnot_gate.target);
                    self.state = cnot_matrix.dot(&self.state);
                }
            }
        }
    }

    fn expand_single_qubit_gate(&self, gate: &QuantumGate, qubit_idx: usize) -> Array2<Complex64> {
        let gate_matrix = gate.matrix();
        let mut expanded_gate = Array2::<Complex64>::eye(1);
        for i in 0..self.num_qubits {
            if i == qubit_idx {
                expanded_gate = kron(&expanded_gate, &gate_matrix);
            } else {
                expanded_gate = kron(&expanded_gate, &Array2::eye(2));
            }
        }
        expanded_gate
    }

    fn construct_cnot_matrix(&self, control: usize, target: usize) -> Array2<Complex64> {
        let dim = 1 << self.num_qubits;
        let mut cnot_matrix = Array2::<Complex64>::eye(dim);

        for i in 0..dim {
            if (i >> control) & 1 == 1 {
                let j = i ^ (1 << target);
                cnot_matrix[[i, i]] = Complex64::new(0.0, 0.0);
                cnot_matrix[[j, i]] = Complex64::new(1.0, 0.0);
            }
        }
        cnot_matrix
    }
}

enum Operation {
    SingleQubitGate(QuantumGate, usize),
    CNOTGate(CNOTGate),
}

struct CNOTGate {
    control: usize,
    target: usize,
}

fn main() {
    let mut circuit = QuantumCircuit::new(3);

    circuit.add_gate(QuantumGate::H, 0);
    //circuit.add_gate(QuantumGate::H, 1);
    circuit.add_gate(QuantumGate::H, 2);
    circuit.add_cnot(0, 1);
    circuit.add_cnot(1, 2);

    circuit.execute();

    for (i, amp) in circuit.state.iter().enumerate() {
        println!("State |{}⟩: {:.2} + {:.2}i", i, amp.re, amp.im);
    }
}
