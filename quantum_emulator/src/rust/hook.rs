use crate::logic::{Circuit, Gate, GateType};
use rocket::serde::Deserialize;

/// Represents the JSON structure of a quantum circuit.
#[derive(Deserialize)]
pub struct JSONCircuit {
    pub qubits: usize,
    pub gates: Vec<JSONGate>,
}

/// Represents the JSON structure of a single gate within the circuit.
#[derive(Deserialize)]
pub struct JSONGate {
    #[serde(rename = "type")]
    pub gate_type: String,
    pub q: usize,
    pub t: usize,
    pub id: usize,
    pub theta: Option<String>,
    #[serde(rename = "thetaValue")]
    pub thetaValue: Option<f64>,
    pub controls: Option<Vec<usize>>,
    pub twoQubits: Option<Vec<usize>>,
}

/// Parses the theta value from a string to a floating-point number.
///
/// # Panics
///
/// Panics if the string cannot be parsed into a `f64`.
fn parse_theta(theta_str: &str) -> f64 {
    theta_str
        .parse::<f64>()
        .expect("Failed to parse theta as a float")
}

/// Converts the gate type from its string representation to the corresponding `GateType` enum.
///
/// Handles rotation gates (`RX`, `RY`, `RZ`) by parsing the theta value.
///
/// # Panics
///
/// Panics if a rotation gate is missing a theta value or if an unknown gate type is encountered.
fn parse_gate_type(json_gate: &JSONGate) -> Option<GateType> {
    match json_gate.gate_type.as_str() {
        "X" => Some(GateType::X),
        "Y" => Some(GateType::Y),
        "Z" => Some(GateType::Z),
        "S" => Some(GateType::S),
        "H" => Some(GateType::H),
        "T" => Some(GateType::T),
        "RX" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(
                    json_gate
                        .theta
                        .as_ref()
                        .expect("Theta value is missing for RX gate"),
                )
            });
            Some(GateType::Rx(theta))
        }
        "RY" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(
                    json_gate
                        .theta
                        .as_ref()
                        .expect("Theta value is missing for RY gate"),
                )
            });
            Some(GateType::Ry(theta))
        }
        "RZ" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(
                    json_gate
                        .theta
                        .as_ref()
                        .expect("Theta value is missing for RZ gate"),
                )
            });
            Some(GateType::Rz(theta))
        }
        "CNOT" => Some(GateType::CNOT),
        "SWAP" => Some(GateType::SWAP),
        "Q" => None, // Ignore 'Q' gates
        other => panic!("Unknown gate type: {}", other),
    }
}

/// Converts a `JSONGate` to the internal `Gate` structure.
///
/// Returns `None` if the gate type is to be ignored (e.g., "Q" gates).
///
/// # Panics
///
/// Panics if required fields for certain gate types are missing.
fn convert_json_gate(json_gate: &JSONGate) -> Option<Gate> {
    parse_gate_type(json_gate).map(|gate_type| {
        let qubits = match gate_type {
            GateType::CNOT => {
                let controls = json_gate
                    .controls
                    .as_ref()
                    .expect("CNOT gate missing controls");
                vec![controls[0], json_gate.q]
            }
            GateType::SWAP => {
                let two_qubits = json_gate
                    .twoQubits
                    .as_ref()
                    .expect("SWAP gate missing second qubit");
                vec![two_qubits[0], two_qubits[1]]
            }
            _ => vec![json_gate.q],
        };

        Gate { gate_type, qubits }
    })
}

/// Converts a `JSONCircuit` to the internal `Circuit` structure.
///
/// Filters out any gates that are ignored during conversion.
pub fn convert_json_circuit(json_circuit: JSONCircuit) -> Circuit {
    let gates = json_circuit
        .gates
        .iter()
        .filter_map(convert_json_gate)
        .collect();

    Circuit {
        n_qubits: json_circuit.qubits,
        gates,
    }
}
