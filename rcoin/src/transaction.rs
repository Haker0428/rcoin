use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub txid: String,    // 引用的交易哈希，对应bitcoin中引用的UTXO
    pub vout: usize,     // 引用交易的输出索引
    pub signature: String, // 签名
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {
    pub value: u64,      // 输出金额w
    pub pubkey: String,  // 接收方公钥
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub version: i64, // 交易
    pub inputs: Vec<TransactionInput>,  // 输入集合
    pub outputs: Vec<TransactionOutput>, // 输出集合
    pub timestamp: u64,                 // 时间戳
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_input_serialization() {
        let input = TransactionInput {
            txid: "test_txid".to_string(),
            vout: 0,
            signature: "test_signature".to_string(),
        };
        let serialized = serde_json::to_string(&input).unwrap();
        let deserialized: TransactionInput = serde_json::from_str(&serialized).unwrap();
        assert_eq!(input.txid, deserialized.txid);
        assert_eq!(input.vout, deserialized.vout);
        assert_eq!(input.signature, deserialized.signature);
    }

    #[test]
    fn test_transaction_output_serialization() {
        let output = TransactionOutput {
            value: 1000,
            pubkey: "test_pubkey".to_string(),
        };
        let serialized = serde_json::to_string(&output).unwrap();
        let deserialized: TransactionOutput = serde_json::from_str(&serialized).unwrap();
        assert_eq!(output.value, deserialized.value);
        assert_eq!(output.pubkey, deserialized.pubkey);
    }

    #[test]
    fn test_transaction_serialization() {
        let transaction = Transaction {
            inputs: vec![
                TransactionInput {
                    txid: "test_txid".to_string(),
                    vout: 0,
                    signature: "test_signature".to_string(),
                },
            ],
            outputs: vec![
                TransactionOutput {
                    value: 1000,
                    pubkey: "test_pubkey".to_string(),
                },
            ],
            timestamp: 1625251234,
        };
        let serialized = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(transaction.inputs.len(), deserialized.inputs.len());
        assert_eq!(transaction.outputs.len(), deserialized.outputs.len());
        assert_eq!(transaction.timestamp, deserialized.timestamp);
    }
}
