// Assume we have a module that handles dispute resolution.
pub mod dispute_resolution {
    // ... Other necessary imports and definitions

    // This function is called when a dispute arises and the proof needs to be verified on-chain.
    pub fn on_chain_dispute_verification(proof: &Proof, disputed_computation: &Computation) -> Result<(), DisputeError> {
        // Check if the proof size is within the block size limit.
        if proof.size() > BITCOIN_MAX_BLOCK_SIZE {
            return Err(DisputeError::ProofTooLarge);
        }

        // Execute the disputed computation on-chain.
        // This is a simplified example. Actual implementation would depend on the specifics of the proof and computation.
        match execute_bitcoin_script(&proof) {
            Ok(_) => Ok(()), // The proof is valid, and the dispute is resolved.
            Err(error) => Err(DisputeError::InvalidProof(error)), // The proof is invalid, and the prover loses the dispute.
        }
    }

    // Function to execute a bitcoin script on-chain (a placeholder for actual implementation).
    fn execute_bitcoin_script(proof: &Proof) -> Result<(), BitcoinScriptError> {
        // Implementation of bitcoin script execution. This should be designed to fit within a single block.
        // ...
    }

    // Custom error types for dispute resolution.
    #[derive(Debug)]
    pub enum DisputeError {
        ProofTooLarge,
        InvalidProof(BitcoinScriptError),
        // Other error types...
    }

    #[derive(Debug)]
    pub enum BitcoinScriptError {
        ExecutionFailed,
        // Other error types...
    }
}
