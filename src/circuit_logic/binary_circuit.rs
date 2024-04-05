use serde_json::Value;
use crate::logic_gate::{LogicGate, GateType};

pub struct BinaryCircuit {
    gates: Vec<LogicGate>,
    connections: Vec<(usize, usize)>, // (output_gate_index, input_gate_index)
}

impl BinaryCircuit {
    pub fn new() -> Self {
        BinaryCircuit {
            gates: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn from_json(json_data: &str) -> Result<Self, serde_json::Error> {
        let json: Value = serde_json::from_str(json_data)?;
        let mut circuit = BinaryCircuit::new();

        if let Some(components) = json.as_array() {
            for component in components {
                // Parse each component and add to the circuit

                // Handle AND gates
                if let Some(gate_type) = component["type"].as_str() {
                    if gate_type == "AND" {
                        let inputs = component["inputs"].as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_u64())
                            .map(|v| v as usize)
                            .collect();

                        let gate = LogicGate::new(GateType::AND, inputs);
                        circuit.gates.push(gate);
                    }
                }

                // Handle XOR gates
                if let Some(gate_type) = component["type"].as_str() {
                    if gate_type == "XOR" {
                        let inputs = component["inputs"].as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_u64())
                            .map(|v| v as usize)
                            .collect();

                        let gate = LogicGate::new(GateType::XOR, inputs);
                        circuit.gates.push(gate);
                    }
                }

                // Handle NOT gates
                if let Some(gate_type) = component["type"].as_str() {
                    if gate_type == "NOT" {
                        let inputs = component["inputs"].as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_u64())
                            .map(|v| v as usize)
                            .collect();

                        let gate = LogicGate::new(GateType::NOT, inputs);
                        circuit.gates.push(gate);
                    }
                }

                // Add logic for connections (if defined in JSON)
            }
        }

        Ok(circuit)
    }

    // Additional methods for circuit manipulation
}
