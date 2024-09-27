# logic.py
import numpy as np

class GateType:
    X = 'X'
    Y = 'Y'
    Z = 'Z'
    S = 'S'
    H = 'H'
    T = 'T'
    Rx = 'Rx'
    Ry = 'Ry'
    Rz = 'Rz'
    CNOT = 'CNOT'

def get_single_qubit_gate_matrix(gate_type, theta=None):
    if gate_type == GateType.X:
        return np.array([[0, 1], [1, 0]], dtype=complex)
    elif gate_type == GateType.Y:
        return np.array([[0, -1j], [1j, 0]], dtype=complex)
    elif gate_type == GateType.Z:
        return np.array([[1, 0], [0, -1]], dtype=complex)
    elif gate_type == GateType.S:
        return np.array([[1, 0], [0, 1j]], dtype=complex)
    elif gate_type == GateType.H:
        return (1/np.sqrt(2)) * np.array([[1, 1], [1, -1]], dtype=complex)
    elif gate_type == GateType.T:
        return np.array([[1, 0], [0, np.exp(1j * np.pi / 4)]], dtype=complex)
    elif gate_type == GateType.Rx:
        return np.array([
            [np.cos(theta/2), -1j * np.sin(theta/2)],
            [-1j * np.sin(theta/2), np.cos(theta/2)]
        ], dtype=complex)
    elif gate_type == GateType.Ry:
        return np.array([
            [np.cos(theta/2), -np.sin(theta/2)],
            [np.sin(theta/2), np.cos(theta/2)]
        ], dtype=complex)
    elif gate_type == GateType.Rz:
        return np.array([
            [np.exp(-1j * theta / 2), 0],
            [0, np.exp(1j * theta / 2)]
        ], dtype=complex)
    else:
        raise ValueError(f"Unknown gate type: {gate_type}")

def get_cnot_matrix(n_qubits, control, target):
    dim = 2 ** n_qubits
    matrix = np.zeros((dim, dim), dtype=complex)

    for i in range(dim):
        bits = [(i >> q) & 1 for q in range(n_qubits)]
        if bits[control] == 1:
            bits[target] ^= 1
        j = sum(bit << q for q, bit in enumerate(bits))
        matrix[j, i] = 1
    return matrix

class Gate:
    def __init__(self, gate_type, qubits, theta=None):
        self.gate_type = gate_type
        self.qubits = qubits
        self.theta = theta

    def get_full_unitary(self, n_qubits):
        if self.gate_type == GateType.CNOT:
            control, target = self.qubits
            return get_cnot_matrix(n_qubits, control, target)
        else:
            gate_matrix = get_single_qubit_gate_matrix(self.gate_type, self.theta)
            matrices = []
            for q in range(n_qubits):
                if q in self.qubits:
                    matrices.append(gate_matrix)
                else:
                    matrices.append(np.identity(2, dtype=complex))
            full_matrix = matrices[0]
            for m in matrices[1:]:
                full_matrix = np.kron(full_matrix, m)
            return full_matrix

class Circuit:
    def __init__(self, n_qubits, gates):
        self.n_qubits = n_qubits
        self.gates = gates

    def get_unitary_matrix(self):
        dim = 2 ** self.n_qubits
        U = np.identity(dim, dtype=complex)
        for gate in self.gates:
            U_gate = gate.get_full_unitary(self.n_qubits)
            U = U_gate @ U
        return U

    def get_state_vector(self):
        dim = 2 ** self.n_qubits
        state = np.zeros(dim, dtype=complex)
        state[0] = 1.0
        U = self.get_unitary_matrix()
        state = U @ state
        return state

    def get_basis_vectors(self):
        return [format(i, f'0{self.n_qubits}b') for i in range(2 ** self.n_qubits)]

def parse_theta(theta_str):
    try:
        # Replace 'π' with 'np.pi'
        expression = theta_str.replace('π', 'np.pi')
        return eval(expression)
    except Exception as e:
        raise ValueError(f"Invalid theta expression: {theta_str}") from e

def convert_json_circuit(json_circuit):
    n_qubits = json_circuit['qubits']
    gates = []
    for json_gate in json_circuit['gates']:
        gate_type = json_gate['type']

        # Skip 'Q' gates
        if gate_type == 'Q':
            continue

        q = json_gate['q']
        theta = None
        if gate_type in ['RX', 'RY', 'RZ']:
            theta_value = json_gate.get('thetaValue')
            if theta_value is not None:
                theta = theta_value
            else:
                theta = parse_theta(json_gate.get('theta'))
        if gate_type == 'CNOT':
            controls = json_gate.get('controls')
            if controls is None or len(controls) == 0:
                raise ValueError("CNOT gate requires control qubits")
            qubits = [controls[0], q]
        else:
            qubits = [q]
        gate = Gate(gate_type, qubits, theta)
        gates.append(gate)
    return Circuit(n_qubits, gates)
