pub struct BitcoinTransaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

pub struct TxInput {
    pub prev_hash: String,    // Previous transaction hash (hex format)
    pub output_index: u32,    // Output index in the previous transaction
    pub script_sig: String,   // ScriptSig (hex format)
    pub sequence: u32,        // Sequence number
}

pub struct TxOutput {
    pub value: u64,           // Value in satoshis
    pub script_pubkey: String,// ScriptPubKey (hex format)
}

pub struct BristolEncoder;

impl BristolEncoder {
    pub fn new() -> Self {
        BristolEncoder
    }

    pub fn encode_transaction(&self, transaction: &BitcoinTransaction) -> Vec<u8> {
        let mut encoded = Vec::new();
        
        for input in &transaction.inputs {
            encoded.extend(self.encode_input(input));
        }

        for output in &transaction.outputs {
            encoded.extend(self.encode_output(output));
        }

        encoded
    }

    fn encode_input(&self, input: &TxInput) -> Vec<u8> {
        let mut encoded = Vec::new();

        encoded.extend(self.encode_hash(&input.prev_hash));
        encoded.extend(input.output_index.to_le_bytes().to_vec());
        encoded.extend(self.encode_var_length_data(&input.script_sig));
        encoded.extend(input.sequence.to_le_bytes().to_vec());

        encoded
    }

    fn encode_output(&self, output: &TxOutput) -> Vec<u8> {
        let mut encoded = Vec::new();

        encoded.extend(output.value.to_le_bytes().to_vec());
        encoded.extend(self.encode_var_length_data(&output.script_pubkey));

        encoded
    }

    fn encode_hash(&self, hash: &str) -> Vec<u8> {
        // Convert the hex string hash into bytes
        hex::decode(hash).unwrap_or_else(|_| vec![])
    }

    fn encode_var_length_data(&self, data: &str) -> Vec<u8> {
        // Convert the hex string data into bytes
        hex::decode(data).unwrap_or_else(|_| vec![])
    }
}
