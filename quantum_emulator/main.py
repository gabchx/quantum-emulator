# main.py
from flask import Flask, request, jsonify, send_from_directory, redirect, url_for
from flask_cors import CORS
import os
from logic import Circuit, convert_json_circuit

app = Flask(__name__)
CORS(app, resources={r"/*": {"origins": "*"}})

@app.route('/simulate', methods=['POST', 'OPTIONS'])
def simulate():
    if request.method == 'OPTIONS':
        return '', 200

    json_circuit = request.get_json()
    circuit = convert_json_circuit(json_circuit)
    state_vector = circuit.get_state_vector()
    basis_vectors = circuit.get_basis_vectors()

    # Convert complex numbers to serializable format
    state_vector_serializable = [(c.real, c.imag) for c in state_vector]
    response = {
        "state_vector": state_vector_serializable,
        "basis_vectors": basis_vectors,
    }

    return jsonify(response)

if __name__ == '__main__':
    app.run(debug=True)
