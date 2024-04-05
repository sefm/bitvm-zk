use risc0_zkvm::zkvm::{Prover, Verifier, Receipt};
use crate::circuit_logic::BinaryCircuit;
use std::io::Cursor;

pub struct ZkProofs;

impl ZkProofs {
    pub fn new() -> Self {
        ZkProofs
    }

    pub fn generate_proof(&self, circuit: &BinaryCircuit) -> Result<Receipt, String> {
        let mut circuit_bytes = Vec::new();
        for gate in &circuit.gates {
            // Serialize each gate
            circuit_bytes.extend_from_slice(&gate.serialize());
        }

        let mut prover = Prover::new();
        let mut cursor = Cursor::new(circuit_bytes);
        prover.add_input(&mut cursor).map_err(|e| e.to_string())?;
        prover.run().map_err(|e| e.to_string())
    }

    pub fn verify_proof(&self, receipt: &Receipt) -> Result<(), String> {
        let mut verifier = Verifier::new();
        verifier.verify(receipt).map_err(|e| e.to_string())
    }
}
