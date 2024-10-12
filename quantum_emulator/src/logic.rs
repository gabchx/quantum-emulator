use nalgebra::{DMatrix, Matrix2};
use num_complex::Complex;
use std::f64::consts::PI;

/// Represents the various types of quantum gates.
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
    SWAP,
}

/// Represents a quantum gate, including its type and the qubits it acts upon.
#[derive(Clone, Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub qubits: Vec<usize>,
}

/// Represents a quantum circuit, consisting of multiple gates and the number of qubits.
#[derive(Debug)]
pub struct Circuit {
    pub n_qubits: usize,
    pub gates: Vec<Gate>,
}

impl GateType {
    /// Returns the 2x2 unitary matrix corresponding to the gate type.
    ///
    /// # Panics
    ///
    /// Panics if the gate type is `CNOT` or `SWAP` since they are not single-qubit gates.
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
            GateType::SWAP => {
                panic!("SWAP gate is not a single-qubit gate.");
            }
        }
    }
}

impl Gate {
    /// Returns the full unitary matrix for this gate within a system of `n_qubits`.
    ///
    /// For single-qubit gates, it constructs the tensor product appropriately.
    /// For multi-qubit gates like CNOT and SWAP, it uses specialized functions.
    ///
    /// # Arguments
    ///
    /// * `n_qubits` - The total number of qubits in the circuit.
    ///
    /// # Panics
    ///
    /// Panics if the gate type is unrecognized or if multi-qubit gates are not properly handled.
    pub fn get_full_unitary(&self, n_qubits: usize) -> DMatrix<Complex<f64>> {
        match &self.gate_type {
            GateType::CNOT => {
                let control = self.qubits[0];
                let target = self.qubits[1];
                get_cnot_matrix(n_qubits, control, target)
            }
            GateType::SWAP => {
                let qubit1 = self.qubits[0];
                let qubit2 = self.qubits[1];
                get_swap_matrix(n_qubits, qubit1, qubit2)
            }
            _ => {
                let gate_matrix = self.gate_type.unitary_matrix();
                let mut matrices = Vec::with_capacity(n_qubits);

                for q in 0..n_qubits {
                    if self.qubits.contains(&(n_qubits - q - 1)) {
                        matrices.push(DMatrix::from_row_slice(2, 2, gate_matrix.as_slice()));
                    } else {
                        matrices.push(DMatrix::<Complex<f64>>::identity(2, 2));
                    }
                }

                matrices
                    .iter()
                    .rev() // Reverse to match qubit ordering
                    .fold(DMatrix::identity(1, 1), |acc, m| kronecker_product(&acc, m))
            }
        }
    }
}

impl Circuit {
    /// Computes the overall unitary matrix of the entire circuit.
    ///
    /// This is achieved by sequentially applying each gate's unitary matrix.
    ///
    /// # Returns
    ///
    /// A `DMatrix` representing the unitary matrix of the circuit.
    pub fn get_unitary_matrix(&self) -> DMatrix<Complex<f64>> {
        let dim = 1 << self.n_qubits;
        let mut u = DMatrix::<Complex<f64>>::identity(dim, dim);

        for gate in &self.gates {
            let u_gate = gate.get_full_unitary(self.n_qubits);
            u = &u_gate * u;
        }

        u
    }

    /// Computes the final state vector of the circuit starting from the |0...0⟩ state.
    ///
    /// # Returns
    ///
    /// A `DMatrix` representing the state vector.
    pub fn get_state_vector(&self) -> DMatrix<Complex<f64>> {
        let dim = 1 << self.n_qubits;
        let initial_state = DMatrix::<Complex<f64>>::from_element(dim, 1, Complex::new(0.0, 0.0));
        let mut state = initial_state;
        state[(0, 0)] = Complex::new(1.0, 0.0); // |0...0⟩ state

        let u = self.get_unitary_matrix();
        u * state
    }

    /// Generates a list of basis vectors in binary string format.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` where each string represents a basis vector.
    pub fn get_basis_vectors(&self) -> Vec<String> {
        (0..(1 << self.n_qubits))
            .map(|i| format!("{:0width$b}", i, width = self.n_qubits))
            .collect()
    }

    /// Computes the probability of each basis state in the final state vector.
    ///
    /// # Returns
    ///
    /// A `Vec<f64>` containing the probabilities.
    pub fn get_state_probabilities(&self) -> Vec<f64> {
        self.get_state_vector()
            .iter()
            .map(|amplitude| amplitude.norm_sqr())
            .collect()
    }
}

/// Computes the Kronecker (tensor) product of two matrices.
///
/// # Arguments
///
/// * `a` - The first matrix.
/// * `b` - The second matrix.
///
/// # Returns
///
/// A `DMatrix` representing the Kronecker product of `a` and `b`.
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

/// Generates the unitary matrix for a CNOT gate within an `n_qubits` system.
///
/// # Arguments
///
/// * `n_qubits` - Total number of qubits in the system.
/// * `control` - Index of the control qubit.
/// * `target` - Index of the target qubit.
///
/// # Returns
///
/// A `DMatrix` representing the CNOT gate's unitary matrix.
pub fn get_cnot_matrix(n_qubits: usize, control: usize, target: usize) -> DMatrix<Complex<f64>> {
    let dim = 1 << n_qubits;
    let mut cnot_matrix = DMatrix::<Complex<f64>>::from_element(dim, dim, Complex::new(0.0, 0.0));

    for i in 0..dim {
        let control_bit = (i >> control) & 1;
        let mut j = i;

        if control_bit == 1 {
            j ^= 1 << target; // Flip target qubit
        }

        cnot_matrix[(i, j)] = Complex::new(1.0, 0.0);
    }

    cnot_matrix
}

/// Generates the unitary matrix for a SWAP gate within an `n_qubits` system.
///
/// # Arguments
///
/// * `n_qubits` - Total number of qubits in the system.
/// * `qubit1` - Index of the first qubit to swap.
/// * `qubit2` - Index of the second qubit to swap.
///
/// # Returns
///
/// A `DMatrix` representing the SWAP gate's unitary matrix.
pub fn get_swap_matrix(n_qubits: usize, qubit1: usize, qubit2: usize) -> DMatrix<Complex<f64>> {
    let dim = 1 << n_qubits;
    let mut swap_matrix = DMatrix::<Complex<f64>>::from_element(dim, dim, Complex::new(0.0, 0.0));

    for i in 0..dim {
        let bit_a = (i >> qubit1) & 1;
        let bit_b = (i >> qubit2) & 1;
        let mut j = i;

        if bit_a != bit_b {
            j ^= 1 << qubit1;
            j ^= 1 << qubit2;
        }

        swap_matrix[(i, j)] = Complex::new(1.0, 0.0);
    }

    swap_matrix
}

/// Computes the Bloch sphere angles (theta, phi) for each qubit in the state vector.
///
/// # Arguments
///
/// * `state_vector` - The state vector of the system.
/// * `n_qubits` - Total number of qubits.
///
/// # Returns
///
/// A `Vec<(f64, f64)>` where each tuple contains the theta and phi angles for a qubit.
pub fn bloch_sphere_angles_per_qubit(
    state_vector: &DMatrix<Complex<f64>>,
    n_qubits: usize,
) -> Vec<(f64, f64)> {
    (0..n_qubits)
        .map(|qubit| {
            let (alpha, beta) = extract_single_qubit_amplitudes(state_vector, n_qubits, qubit);

            // Compute Bloch angles
            let theta = if beta.norm() == 0.0 {
                0.0
            } else if alpha.norm() == 0.0 {
                PI
            } else {
                2.0 * alpha.norm().acos()
            };

            let phi = if alpha.norm() == 0.0 && beta.norm() == 0.0 {
                0.0
            } else {
                beta.arg() - alpha.arg()
            };

            (theta, phi)
        })
        .collect()
}

/// Extracts the amplitudes for a single qubit from the full state vector.
///
/// # Arguments
///
/// * `state_vector` - The state vector of the system.
/// * `n_qubits` - Total number of qubits.
/// * `target_qubit` - The qubit for which to extract amplitudes.
///
/// # Returns
///
/// A tuple `(alpha, beta)` representing the amplitudes for |0⟩ and |1⟩ states of the target qubit.
fn extract_single_qubit_amplitudes(
    state_vector: &DMatrix<Complex<f64>>,
    n_qubits: usize,
    target_qubit: usize,
) -> (Complex<f64>, Complex<f64>) {
    let dim = 1 << n_qubits;
    let mut alpha = Complex::new(0.0, 0.0);
    let mut beta = Complex::new(0.0, 0.0);

    for i in 0..dim {
        let qubit_state = (i >> target_qubit) & 1;
        if qubit_state == 0 {
            alpha += state_vector[(i, 0)];
        } else {
            beta += state_vector[(i, 0)];
        }
    }

    (alpha, beta)
}
