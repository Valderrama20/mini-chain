use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::tx::Transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: i64,
    pub hash: String,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        // creamos un string con la informacion para el hash
        let data = format!(
            "{}{:?}{}{}",
            self.timestamp, self.transactions, self.previous_hash, self.nonce
        );

        // Creamos una instancia del creador de hash
        let mut hasher = Sha256::new();

        // Ingresamos la informacion al hashear tranformada en una vector de bytes
        hasher.update(data.as_bytes());

        // Creamos el Hash
        let result = hasher.finalize();

        // tranformamos el hash a hexadecimal y retornamos
        hex::encode(result)
    }
}
