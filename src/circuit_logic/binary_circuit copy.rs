use crate::logic_gate::{LogicGate, GateType};

pub struct BinaryCircuit {
    gates: Vec<LogicGate>,
    connections: Vec<(usize, usize)>, // (output_gate_index, input_gate_index)
}

impl BinaryCircuit {
    /// Creates a new, empty BinaryCircuit.
    pub fn new() -> Self {
        BinaryCircuit {
            gates: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Adds a logic gate to the circuit.
    pub fn add_gate(&mut self, gate: LogicGate) -> usize {
        let index = self.gates.len();
        self.gates.push(gate);
        index
    }

    /// Adds a connection between two gates in the circuit.
    pub fn add_connection(&mut self, output_gate_index: usize, input_gate_index: usize) {
        self.connections.push((output_gate_index, input_gate_index));
    }

    /// Parses a circuit from a string in the Bristol Fashion format.
    pub fn from_bristol_format(data: &str) -> Result<Self, String> {
        let mut circuit = BinaryCircuit::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue; // Skip empty or invalid lines
            }

            let gate_type = match parts[2] {
                "AND" => GateType::AND,
                "NAND" => GateType::NAND,
                "NOT" => GateType::NOT,
                // Handle XOR using a combination of AND and NOT gates
                // Inside the from_bristol_format method, within the gate type matching:

                "XOR" => {
                    // Example: Construct XOR using NAND, AND, and NOT
                    // First, create necessary gates and add them to the circuit
                    let nand_gate_index = circuit.add_gate(LogicGate::new(GateType::NAND, vec![input1, input2]));
                    let not_gate1_index = circuit.add_gate(LogicGate::new(GateType::NOT, vec![input1]));
                    let not_gate2_index = circuit.add_gate(LogicGate::new(GateType::NOT, vec![input2]));
                    let and_gate1_index = circuit.add_gate(LogicGate::new(GateType::AND, vec![not_gate1_index, input2]));
                    let and_gate2_index = circuit.add_gate(LogicGate::new(GateType::AND, vec![input1, not_gate2_index]));

                    // Combine the gates to form XOR
                    let xor_output = /* logic to combine the gates */;
                    circuit.add_gate(LogicGate::new(GateType::OR, vec![and_gate1_index, and_gate2_index]));

                    // Add connections
                    circuit.add_connection(nand_gate_index, xor_output);
                    // ... other connections for XOR construction ...
                    continue;
                }
                _ => return Err("Unknown gate type".to_string()),
            };

            let inputs = parts[3..].iter()
                            .filter_map(|&s| s.parse::<usize>().ok())
                            .collect::<Vec<_>>();

            let gate_index = circuit.add_gate(LogicGate::new(gate_type, inputs));

            // Assuming the last part is the output index for the gate
            if let Some(&output_part) = parts.last() {
                if let Ok(output_index) = output_part.parse::<usize>() {
                    circuit.add_connection(gate_index, output_index);
                }
            }
        }

        Ok(circuit)
    }

    pub fn create_op_code_circuit(op_code: &str) -> Result<Self, String> {
        match op_code {
            "OP_ADD" => Ok(op_code_circuits::op_add_circuit()),
            "OP_VERIFY" => Ok(op_code_circuits::op_verify_circuit()),
            "OP_SIGN" => Ok(op_code_circuits::op_sign_circuit()),
            "OP_SHA256" => Ok(op_code_circuits::op_sha256_circuit()),
            // Add cases for other OP_CODES
            _ => Err("Unknown OP_CODE".to_string()),
        }
    }

    impl BinaryCircuit {
        /// Creates a static circuit for OP_ADD.
        fn create_add_circuit() -> Result<Self, String> {
            let mut circuit = BinaryCircuit::new();
            let a_index = circuit.add_gate(LogicGate::new(GateType::Input, vec![])); // A
            let b_index = circuit.add_gate(LogicGate::new(GateType::Input, vec![])); // B
            let c_index = circuit.add_gate(LogicGate::new(GateType::Input, vec![])); // C
            let d_index = circuit.add_gate(LogicGate::new(GateType::Input, vec![])); // D
    
            let nand_cd_index = circuit.add_gate(LogicGate::new(GateType::NAND, vec![c_index, d_index]));
            let nand_b_nand_cd_index = circuit.add_gate(LogicGate::new(GateType::NAND, vec![b_index, nand_cd_index]));
            let add_output_index = circuit.add_gate(LogicGate::new(GateType::NAND, vec![a_index, nand_b_nand_cd_index]));
    
            // Set connections according to the logic
            circuit.add_connection(c_index, nand_cd_index);
            circuit.add_connection(d_index, nand_cd_index);
            circuit.add_connection(b_index, nand_b_nand_cd_index);
            circuit.add_connection(nand_cd_index, nand_b_nand_cd_index);
            circuit.add_connection(a_index, add_output_index);
            circuit.add_connection(nand_b_nand_cd_index, add_output_index);
    
            Ok(circuit)
        }
    
        // ... other methods ...
    }
    
    /// Serializes the circuit for RISC0.
    pub fn serialize_for_risc0(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for gate in &self.gates {
            bytes.extend(gate.serialize_for_risc0());
        }
        // Serialize connections and other relevant data for RISC0
        for &(output, input) in &self.connections {
            bytes.extend(&output.to_le_bytes());
            bytes.extend(&input.to_le_bytes());
        }
        bytes
    }
}
