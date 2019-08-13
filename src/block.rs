use crate::types::{Block, Transaction};
use sha2::{Digest, Sha256};

impl Block {
  pub fn new(
    _block_number: u64,
    _nonce: u64,
    _coinbase: std::string::String,
    _transaction: Transaction,
    _prev_hash: std::string::String,
    _hash: std::string::String,
  ) -> Block {
    Block {
      block_number: _block_number,
      nonce: _nonce,
      coinbase: _coinbase,
      transaction: _transaction,
      prev_hash: _prev_hash,
      hash: _hash,
    }
  }

  // Satoshi Nakamoto Proof Of Work Consensus
  pub fn calculate_hash(&self) {
    let mut hasher = sha2::Sha256::new();
    hasher.input("");
    println!("{}", self.hash);
  }
}