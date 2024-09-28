from flask import Flask, request, jsonify
from flask_cors import CORS
from python_backend.logic import convert_json_circuit

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

def run():
    app.run(debug=True, port=8001)
