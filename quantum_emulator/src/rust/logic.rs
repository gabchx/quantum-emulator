// logic.rs
use nalgebra::{DMatrix, Matrix2};
use num_complex::Complex;
use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub enum GateType {
    X,
    Y,
    Z,
    S,
    H,
    T,
    Rx(f64),
    Ry(f64),
    Rz(f64),
    CNOT,
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub qubits: Vec<usize>,
}

#[derive(Debug)]
pub struct Circuit {
    pub n_qubits: usize,
    pub gates: Vec<Gate>,
}

impl GateType {
    pub fn unitary_matrix(&self) -> Matrix2<Complex<f64>> {
        match self {
            GateType::X => Matrix2::new(
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
            ),
            GateType::Y => Matrix2::new(
                Complex::new(0.0, 0.0),
                Complex::new(0.0, -1.0),
                Complex::new(0.0, 1.0),
                Complex::new(0.0, 0.0),
            ),
            GateType::Z => Matrix2::new(
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(-1.0, 0.0),
            ),
            GateType::S => Matrix2::new(
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 1.0),
            ),
            GateType::H => {
                let inv_sqrt_2 = 1.0 / (2.0_f64).sqrt();
                Matrix2::new(
                    Complex::new(inv_sqrt_2, 0.0),
                    Complex::new(inv_sqrt_2, 0.0),
                    Complex::new(inv_sqrt_2, 0.0),
                    Complex::new(-inv_sqrt_2, 0.0),
                )
            }
            GateType::T => {
                let t = Complex::from_polar(1.0, PI / 4.0);
                Matrix2::new(
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    t,
                )
            }
            GateType::Rx(theta) => {
                let cos = Complex::new((theta / 2.0).cos(), 0.0);
                let i_sin = Complex::new(0.0, -(theta / 2.0).sin());
                Matrix2::new(cos, i_sin, i_sin, cos)
            }
            GateType::Ry(theta) => {
                let cos = Complex::new((theta / 2.0).cos(), 0.0);
                let sin = Complex::new((theta / 2.0).sin(), 0.0);
                Matrix2::new(cos, -sin, sin, cos)
            }
            GateType::Rz(theta) => {
                let e_minus_i_theta_over_2 = Complex::from_polar(1.0, -theta / 2.0);
                let e_plus_i_theta_over_2 = Complex::from_polar(1.0, theta / 2.0);
                Matrix2::new(
                    e_minus_i_theta_over_2,
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    e_plus_i_theta_over_2,
                )
            }
            GateType::CNOT => {
                panic!("CNOT gate is not a single-qubit gate.");
            }
        }
    }
}

impl Gate {
    pub fn get_full_unitary(&self, n_qubits: usize) -> DMatrix<Complex<f64>> {
        match &self.gate_type {
            GateType::CNOT => {
                let control = self.qubits[0];
                let target = self.qubits[1];
                get_cnot_matrix(n_qubits, control, target)
            }
            _ => {
                let gate_matrix = self.gate_type.unitary_matrix();
                let mut matrices = Vec::new();

                for q in 0..n_qubits {
                    if self.qubits.contains(&(n_qubits - q - 1)) {
                        matrices.push(DMatrix::from_row_slice(2, 2, gate_matrix.as_slice()));
                    } else {
                        matrices.push(DMatrix::<Complex<f64>>::identity(2, 2));
                    }
                }

                let mut full_matrix = matrices[0].clone();
                for m in matrices.iter().skip(1) {
                    full_matrix = kronecker_product(&full_matrix, &m);
                }

                full_matrix
            }
        }
    }
}

impl Circuit {
    pub fn get_unitary_matrix(&self) -> DMatrix<Complex<f64>> {
        let dim = 1 << self.n_qubits;
        let mut u = DMatrix::<Complex<f64>>::identity(dim, dim);

        for gate in &self.gates {
            let u_gate = gate.get_full_unitary(self.n_qubits);
            u = u * u_gate;
        }

        u
    }

    pub fn get_state_vector(&self) -> DMatrix<Complex<f64>> {
        let dim = 1 << self.n_qubits;
        let mut state = DMatrix::<Complex<f64>>::zeros(dim, 1);
        state[(0, 0)] = Complex::new(1.0, 0.0);

        let u = self.get_unitary_matrix();
        state = u * state;

        state
    }

    pub fn get_basis_vectors(&self) -> Vec<String> {
        let mut basis_vectors = Vec::new();
        for i in 0..(1 << self.n_qubits) {
            basis_vectors.push(format!("{:0width$b}", i, width = self.n_qubits));
        }

        basis_vectors
    }

    pub fn get_state_probabilities(&self) -> Vec<f64> {
        let state_vector = self.get_state_vector();
        let probabilities: Vec<f64> = state_vector
            .iter()
            .map(|amplitude| amplitude.norm_sqr()) // Squaring the magnitude to get probability
            .collect();

        probabilities
    }
}

pub fn kronecker_product(
    a: &DMatrix<Complex<f64>>,
    b: &DMatrix<Complex<f64>>,
) -> DMatrix<Complex<f64>> {
    let (rows_a, cols_a) = a.shape();
    let (rows_b, cols_b) = b.shape();

    let rows = rows_a * rows_b;
    let cols = cols_a * cols_b;

    let mut result = DMatrix::<Complex<f64>>::zeros(rows, cols);

    for i in 0..rows_a {
        for j in 0..cols_a {
            let a_ij = a[(i, j)];
            for k in 0..rows_b {
                for l in 0..cols_b {
                    result[(i * rows_b + k, j * cols_b + l)] = a_ij * b[(k, l)];
                }
            }
        }
    }

    result
}

pub fn get_cnot_matrix(n_qubits: usize, control: usize, target: usize) -> DMatrix<Complex<f64>> {
    let dim = 1 << n_qubits;
    let mut matrix = DMatrix::<Complex<f64>>::zeros(dim, dim);

    for i in 0..dim {
        let mut bits = (0..n_qubits).map(|q| (i >> q) & 1).collect::<Vec<_>>();

        if bits[control] == 1 {
            bits[target] ^= 1;
        }

        let j = bits
            .iter()
            .enumerate()
            .fold(0, |acc, (q, &bit)| acc | (bit << q));

        matrix[(j, i)] = Complex::new(1.0, 0.0);
    }

    matrix
}

pub fn bloch_sphere_angles_per_qubit(
    state_vector: &DMatrix<Complex<f64>>,
    n_qubits: usize,
) -> Vec<(f64, f64)> {
    let mut angles = Vec::new();
    let dim = state_vector.nrows();

    for qubit in 0..n_qubits {
        let (alpha, beta) = extract_single_qubit_amplitudes(state_vector, n_qubits, qubit);

        // Compute Bloch angles only if alpha and beta are non-zero
        if alpha.norm() > 0.0 || beta.norm() > 0.0 {
            let theta = 2.0 * alpha.norm().acos();
            let phi = beta.arg() - alpha.arg();
            angles.push((theta, phi));
        }
    }

    angles
}

fn extract_single_qubit_amplitudes(
    state_vector: &DMatrix<Complex<f64>>,
    n_qubits: usize,
    target_qubit: usize,
) -> (Complex<f64>, Complex<f64>) {
    let dim = 1 << n_qubits;
    let mut alpha = Complex::new(0.0, 0.0);
    let mut beta = Complex::new(0.0, 0.0);

    for i in 0..dim {
        // Check if the target qubit is in state |0> or |1>
        let qubit_state = (i >> target_qubit) & 1;
        if qubit_state == 0 {
            alpha += state_vector[(i, 0)];
        } else {
            beta += state_vector[(i, 0)];
        }
    }

    (alpha, beta)
}
