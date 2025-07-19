use chrono::Utc;

use crate::models::{block::Block, tx::Transaction};

mod models;

fn main() {
    let transactions = vec![Transaction {
        sender: String::from("I"),
        recipient: String::from("I"),
        amount: 1.5,
    }];

    let block = Block {
        timestamp: Utc::now().timestamp(),
        transactions,
        previous_hash: String::from("0"),
        nonce: 0,
        hash: String::from("block_hash"),
    };

    let hash = block.calculate_hash();

    println!("{:#?}", hash);

    println!("Bienvenidos a Mini Chain!!");
}
