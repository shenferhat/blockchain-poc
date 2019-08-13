use crate::types::{Node, Transaction, Block};
use sha2::{Digest, Sha256};

pub fn hash(data: std::string::String) -> std::string::String {
  let mut hasher = sha2::Sha256::new();
  hasher.input(data);
  hex::encode(hasher.result())
}

impl Node {
  pub fn create_new_block_with_transaction(&self, _transaction: Transaction, _nonce: u32) -> Block {
    let data = format!(
      "{}{}{}{}",
      _transaction.receiver_hash,
      _transaction.sender_hash,
      _transaction.value.to_string(),
      _nonce.to_string()
    );

    println!("Done");
    Block {
      block_number: 0,
      nonce: 0,
      coinbase: self.address.clone(),
      transaction: _transaction,
      prev_hash: String::from(""),
      hash: hash(data),
    }
  }

  // If first char is bigger than 7
  pub fn block_is_valid(&self, block: Block) -> bool {
    if block.hash.chars().next().unwrap().to_digit(10) >= Some(7) {
      true
    } else {
      false
    }
  }
}
