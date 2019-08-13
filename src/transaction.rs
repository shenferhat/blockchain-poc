extern crate rand;
extern crate hex;
use sha2::{Sha256, Digest};
use rand::Rng;

use crate::types::{Transaction};

impl Transaction {
  pub fn new(
    sender: std::string::String,
    reciever: std::string::String,
    _value: u32,
  ) -> Transaction {
    Transaction {
      sender_hash: sender,
      receiver_hash: reciever,
      value: _value,
    }
  }

  pub fn random() -> Self {
    let mut rng = rand::thread_rng();
    let mut hasher = sha2::Sha256::new();
    let mut hasher2 = sha2::Sha256::new();

    let rand_senderhash = rng.gen::<u32>();
    hasher.input(rand_senderhash.to_string());
    let sender = hasher.result();

    let rand_receiverhash = rng.gen::<u32>();
    hasher2.input(rand_receiverhash.to_string());
    let receiver = hasher2.result();
    Transaction {
      sender_hash: hex::encode(sender),
      receiver_hash: hex::encode(receiver),
      value: rng.gen::<u32>(),
    }
  }
}