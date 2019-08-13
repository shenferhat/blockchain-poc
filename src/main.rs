pub mod types;
pub mod node;
pub mod block;
pub mod transaction;

use crate::types::*;
use crate::node::*;
use crate::block::*;
use crate::transaction::*;

fn main() {
  let coinbase = String::from("1b44edba14486826a8882e9423cd9cdc18e46f8e469dd02301fbb2af3dce23a6");

  // Node creates random transaction
  let rand_transaction = Transaction::random();
  let vec_blockchain = Vec::new();

  let genesis_block = Block {
    block_number: 0,
    nonce: 0,
    coinbase: coinbase.clone(),
    transaction: rand_transaction,
    prev_hash: String::from(""),
    hash: String::from(""),
  };

  let node1 = Node {
    address: coinbase.clone(),  // Node has his own address, as miner
    blockchain: vec_blockchain, // Each node is keeper of the blocks this should be a link list
  };

  let mut block1 = node1.create_new_block_with_transaction(genesis_block.transaction.clone(), 0);
  block1 = node1.create_new_block_with_transaction(genesis_block.transaction.clone(), 1);
  println!("{:?}", node1.block_is_valid(block1));

}
