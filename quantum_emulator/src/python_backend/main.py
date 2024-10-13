from python_backend.logic import JSONGate, JSONCircuit, convert_json_circuit, bloch_sphere_angles_per_qubit
from flask import Flask, request, jsonify, redirect, send_from_directory
from flask_cors import CORS
import os

app = Flask(__name__)
CORS(app)

@app.route('/simulate', methods=['POST', 'OPTIONS'])
def simulate():
    if request.method == 'OPTIONS':
        # Handle preflight request
        response = app.make_response('')
        response.headers['Access-Control-Allow-Methods'] = 'POST, OPTIONS'
        response.headers['Access-Control-Allow-Headers'] = 'Content-Type'
        return response

    data = request.get_json()
    if not data:
        return jsonify({"error": "Invalid JSON data"}), 400

    try:
        gates = []
        for gate_data in data.get('gates', []):
            gate = JSONGate(
                gate_type=gate_data.get('type'),
                q=gate_data.get('q'),
                t=gate_data.get('t'),
                id=gate_data.get('id'),
                theta=gate_data.get('theta'),
                thetaValue=gate_data.get('thetaValue'),
                controls=gate_data.get('controls'),
                twoQubits=gate_data.get('twoQubits')
            )
            gates.append(gate)
        json_circuit = JSONCircuit(
            qubits=data.get('qubits'),
            gates=gates
        )
        circuit = convert_json_circuit(json_circuit)
        state_vector = circuit.get_state_vector()
        basis_vectors = circuit.get_basis_vectors()
        probabilities = circuit.get_state_probabilities()
        bloch_angles = bloch_sphere_angles_per_qubit(state_vector, circuit.n_qubits)

        state_vector_serializable = [(c.real, c.imag) for c in state_vector.flatten()]
        response = {
            "state_vector": state_vector_serializable,
            "basis_vectors": basis_vectors,
            "probabilities": probabilities,
            "bloch_angles": bloch_angles
        }

        return jsonify(response), 200

    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route('/', methods=['GET'])
def index():
    return redirect('/home')

@app.route('/home', methods=['GET'])
def home():
    try:
        current_dir = os.getcwd()
        return send_from_directory(os.path.join(current_dir, 'src', 'front'), 'index.html')
    except Exception as e:
        return jsonify({"error": str(e)}), 500

# =========================
# Running the Flask App
# =========================

def run():
    app.run(host='0.0.0.0', port=8001)
