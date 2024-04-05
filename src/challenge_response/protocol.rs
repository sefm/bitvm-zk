use crate::circuit_logic::BinaryCircuit;

pub struct Challenge {
    // Challenge a specific part of the circuit, such as a gate or a set of gates
    pub gate_indices: Vec<usize>,
}

pub struct Response {
    // Response containing the outputs for the challenged gates
    pub gate_outputs: Vec<bool>,
}

pub struct ChallengeResponseProtocol {
    // Additional properties for managing and tracking challenges
    current_challenge: Option<Challenge>,
}

impl ChallengeResponseProtocol {
    pub fn new() -> Self {
        ChallengeResponseProtocol {
            current_challenge: None,
        }
    }

    // Select a subset of gates in the circuit to challenge
    pub fn issue_challenge(&mut self, circuit: &BinaryCircuit, num_gates_to_challenge: usize) -> Challenge {
        let mut gate_indices = Vec::new();
        for _ in 0..num_gates_to_challenge {
            let gate_index = rand::random::<usize>() % circuit.gates.len();
            gate_indices.push(gate_index);
        }

        let challenge = Challenge { gate_indices };
        self.current_challenge = Some(challenge.clone());
        challenge
    }

    // Verify the response against the current challenge
    pub fn process_response(&self, response: &Response, circuit: &BinaryCircuit) -> bool {
        match &self.current_challenge {
            Some(challenge) => {
                for (&gate_index, &response_output) in challenge.gate_indices.iter().zip(response.gate_outputs.iter()) {
                    if circuit.gates[gate_index].evaluate() != response_output {
                        return false; // Mismatch found, response is invalid
                    }
                }
                true // All responses matched, valid response
            },
            None => false, // No challenge issued, response is invalid
        }
    }
}
