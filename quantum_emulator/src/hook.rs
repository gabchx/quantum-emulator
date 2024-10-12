// hook.rs
use crate::logic::{Circuit, Gate, GateType};
use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct JSONCircuit {
    pub qubits: usize,
    pub gates: Vec<JSONGate>,
}

#[derive(Deserialize)]
pub struct JSONGate {
    #[serde(rename = "type")]
    pub gate_type: String,
    pub q: usize,
    pub t: usize,
    pub id: usize,
    pub theta: Option<String>,
    pub thetaValue: Option<f64>,
    pub controls: Option<Vec<usize>>,
}

pub fn parse_theta(theta_str: &str) -> f64 {
    theta_str
        .parse::<f64>()
        .expect("Failed to parse theta as a float")
}

pub fn parse_gate_type(json_gate: &JSONGate) -> Option<GateType> {
    match json_gate.gate_type.as_str() {
        "X" => Some(GateType::X),
        "Y" => Some(GateType::Y),
        "Z" => Some(GateType::Z),
        "S" => Some(GateType::S),
        "H" => Some(GateType::H),
        "T" => Some(GateType::T),
        "RX" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(&json_gate.theta.as_ref().expect("Theta value is missing"))
            });
            Some(GateType::Rx(theta))
        }
        "RY" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(&json_gate.theta.as_ref().expect("Theta value is missing"))
            });
            Some(GateType::Ry(theta))
        }
        "RZ" => {
            let theta = json_gate.thetaValue.unwrap_or_else(|| {
                parse_theta(&json_gate.theta.as_ref().expect("Theta value is missing"))
            });
            Some(GateType::Rz(theta))
        }
        "CNOT" => Some(GateType::CNOT),
        "Q" => None, // Ignore 'Q' gates
        _ => panic!("Unknown gate type: {}", json_gate.gate_type),
    }
}

pub fn convert_json_gate(json_gate: &JSONGate) -> Option<Gate> {
    if let Some(gate_type) = parse_gate_type(json_gate) {
        let qubits = match &gate_type {
            GateType::CNOT => {
                let control_qubits = json_gate
                    .controls
                    .as_ref()
                    .expect("CNOT gate missing controls");
                vec![control_qubits[0], json_gate.q]
            }
            _ => vec![json_gate.q],
        };

        Some(Gate { gate_type, qubits })
    } else {
        None
    }
}

pub fn convert_json_circuit(json_circuit: JSONCircuit) -> Circuit {
    let gates = json_circuit
        .gates
        .iter()
        .filter_map(|json_gate| convert_json_gate(json_gate))
        .collect();

    Circuit {
        n_qubits: json_circuit.qubits,
        gates,
    }
}
