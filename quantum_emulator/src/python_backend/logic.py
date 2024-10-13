import numpy as np
import math

# Constants
PI = math.pi

# =========================
# Data Classes
# =========================

class JSONGate:
    def __init__(self, gate_type, q, t, id, theta=None, thetaValue=None, controls=None, twoQubits=None):
        self.gate_type = gate_type
        self.q = q
        self.t = t
        self.id = id
        self.theta = theta
        self.thetaValue = thetaValue
        self.controls = controls or []
        self.twoQubits = twoQubits or []

class JSONCircuit:
    def __init__(self, qubits, gates):
        self.qubits = qubits
        self.gates = gates

# =========================
# Gate Types and Logic
# =========================

class GateType:
    def __init__(self, gate_type, theta=None):
        self.gate_type = gate_type
        self.theta = theta

    def unitary_matrix(self):
        if self.gate_type == "X":
            return np.array([[0, 1], [1, 0]], dtype=np.complex128)
        elif self.gate_type == "Y":
            return np.array([[0, -1j], [1j, 0]], dtype=np.complex128)
        elif self.gate_type == "Z":
            return np.array([[1, 0], [0, -1]], dtype=np.complex128)
        elif self.gate_type == "S":
            return np.array([[1, 0], [0, 1j]], dtype=np.complex128)
        elif self.gate_type == "H":
            inv_sqrt_2 = 1.0 / math.sqrt(2)
            return np.array([[inv_sqrt_2, inv_sqrt_2], [inv_sqrt_2, -inv_sqrt_2]], dtype=np.complex128)
        elif self.gate_type == "T":
            t = np.exp(1j * PI / 4)
            return np.array([[1, 0], [0, t]], dtype=np.complex128)
        elif self.gate_type == "RX":
            theta = self.theta
            cos = np.cos(theta / 2)
            sin = -1j * np.sin(theta / 2)
            return np.array([[cos, sin], [sin, cos]], dtype=np.complex128)
        elif self.gate_type == "RY":
            theta = self.theta
            cos = np.cos(theta / 2)
            sin = np.sin(theta / 2)
            return np.array([[cos, -sin], [sin, cos]], dtype=np.complex128)
        elif self.gate_type == "RZ":
            theta = self.theta
            e_minus = np.exp(-1j * theta / 2)
            e_plus = np.exp(1j * theta / 2)
            return np.array([[e_minus, 0], [0, e_plus]], dtype=np.complex128)
        else:
            raise ValueError(f"Unknown gate type: {self.gate_type}")

class Gate:
    def __init__(self, gate_type, qubits):
        self.gate_type = gate_type  # Instance of GateType
        self.qubits = qubits  # List of qubit indices

    def get_full_unitary(self, n_qubits):
        if self.gate_type.gate_type == "CNOT":
            control = self.qubits[0]
            target = self.qubits[1]
            return get_cnot_matrix(n_qubits, control, target)
        elif self.gate_type.gate_type == "SWAP":
            qubit1 = self.qubits[0]
            qubit2 = self.qubits[1]
            return get_swap_matrix(n_qubits, qubit1, qubit2)
        else:
            gate_matrix = self.gate_type.unitary_matrix()
            matrices = []
            # Reverse the qubit order to match Rust's ordering (MSB to LSB)
            for q in reversed(range(n_qubits)):
                if q in self.qubits:
                    matrices.append(gate_matrix)
                else:
                    matrices.append(np.identity(2, dtype=np.complex128))
            full_matrix = matrices[0]
            for m in matrices[1:]:
                full_matrix = np.kron(full_matrix, m)
            return full_matrix

class Circuit:
    def __init__(self, n_qubits, gates):
        self.n_qubits = n_qubits
        self.gates = gates  # List of Gate instances

    def get_unitary_matrix(self):
        dim = 1 << self.n_qubits
        u = np.identity(dim, dtype=np.complex128)
        for gate in self.gates:
            u_gate = gate.get_full_unitary(self.n_qubits)
            u = np.dot(u_gate, u)
        return u

    def get_state_vector(self):
        dim = 1 << self.n_qubits
        state = np.zeros((dim, 1), dtype=np.complex128)
        state[0, 0] = 1.0
        u = self.get_unitary_matrix()
        state = np.dot(u, state)
        return state

    def get_basis_vectors(self):
        basis_vectors = []
        for i in range(1 << self.n_qubits):
            basis_vectors.append(format(i, f'0{self.n_qubits}b'))
        return basis_vectors

    def get_state_probabilities(self):
        state_vector = self.get_state_vector()
        probabilities = np.abs(state_vector.flatten())**2
        return probabilities.tolist()

def kronecker_product(a, b):
    return np.kron(a, b)

def get_cnot_matrix(n_qubits, control, target):
    dim = 1 << n_qubits
    cnot_matrix = np.zeros((dim, dim), dtype=np.complex128)
    for i in range(dim):
        control_bit = (i >> (n_qubits - control - 1)) & 1  # Adjusted for reversed qubit order
        if control_bit == 1:
            modified_i = i ^ (1 << (n_qubits - target - 1))  # Adjusted for reversed qubit order
        else:
            modified_i = i
        cnot_matrix[i, modified_i] = 1.0
    return cnot_matrix

def get_swap_matrix(n_qubits, qubit1, qubit2):
    dim = 1 << n_qubits
    swap_matrix = np.zeros((dim, dim), dtype=np.complex128)
    for i in range(dim):
        bit_a = (i >> (n_qubits - qubit1 - 1)) & 1  # Adjusted for reversed qubit order
        bit_b = (i >> (n_qubits - qubit2 - 1)) & 1  # Adjusted for reversed qubit order
        if bit_a != bit_b:
            swapped_i = i ^ (1 << (n_qubits - qubit1 - 1)) ^ (1 << (n_qubits - qubit2 - 1))  # Adjusted
        else:
            swapped_i = i
        swap_matrix[i, swapped_i] = 1.0
    return swap_matrix

def bloch_sphere_angles_per_qubit(state_vector, n_qubits):
    angles = []
    for qubit in range(n_qubits):
        alpha, beta = extract_single_qubit_amplitudes(state_vector, n_qubits, qubit)
        if np.abs(beta) == 0.0:
            theta = 0.0
        elif np.abs(alpha) == 0.0:
            theta = PI
        else:
            theta = 2.0 * np.arccos(np.abs(alpha))
        phi = np.angle(beta) - np.angle(alpha)
        angles.append((theta, phi))
    return angles

def extract_single_qubit_amplitudes(state_vector, n_qubits, target_qubit):
    dim = 1 << n_qubits
    alpha = 0.0 + 0.0j
    beta = 0.0 + 0.0j
    for i in range(dim):
        qubit_state = (i >> (n_qubits - target_qubit - 1)) & 1  # Adjusted for reversed qubit order
        if qubit_state == 0:
            alpha += state_vector[i, 0]
        else:
            beta += state_vector[i, 0]
    return alpha, beta

# =========================
# Conversion Functions
# =========================

def parse_theta(theta_str):
    try:
        return float(theta_str)
    except ValueError:
        raise ValueError("Failed to parse theta as a float")

def parse_gate_type(json_gate):
    gate_type_str = json_gate.gate_type
    if gate_type_str in ["X", "Y", "Z", "S", "H", "T", "CNOT", "SWAP"]:
        return GateType(gate_type_str)
    elif gate_type_str in ["RX", "RY", "RZ"]:
        if json_gate.thetaValue is not None:
            theta = json_gate.thetaValue
        elif json_gate.theta is not None:
            theta = parse_theta(json_gate.theta)
        else:
            raise ValueError(f"Theta value is missing for {gate_type_str} gate")
        return GateType(gate_type_str, theta)
    elif gate_type_str == "Q":
        return None  # Ignore 'Q' gates
    else:
        raise ValueError(f"Unknown gate type: {gate_type_str}")

def convert_json_gate(json_gate):
    gate_type = parse_gate_type(json_gate)
    if gate_type is None:
        return None
    if gate_type.gate_type == "CNOT":
        if not json_gate.controls:
            raise ValueError("CNOT gate missing controls")
        # Adjust qubit ordering: Control is first, target is second
        qubits = [json_gate.controls[0], json_gate.q]
    elif gate_type.gate_type == "SWAP":
        if not json_gate.twoQubits:
            raise ValueError("SWAP gate missing second qubit")
        # SWAP involves two qubits
        qubits = [json_gate.twoQubits[0], json_gate.twoQubits[1]]
    else:
        qubits = [json_gate.q]
    return Gate(gate_type, qubits)

def convert_json_circuit(json_circuit):
    gates = []
    for gj in json_circuit.gates:
        gate = convert_json_gate(gj)
        if gate:
            gates.append(gate)
    return Circuit(json_circuit.qubits, gates)