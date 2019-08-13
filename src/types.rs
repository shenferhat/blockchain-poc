#[derive(Debug, Clone)]
pub struct Transaction {
  pub sender_hash: std::string::String,   // senders address
  pub receiver_hash: std::string::String, // recievers address
  pub value: u32,                         // value that has been sent
}

pub struct Node {
  pub address: std::string::String, // Node has his own address, as miner
  pub blockchain: Vec<Block>,       // Each node is keeper of the blocks this should be a link list
}

pub struct Block {
  pub block_number: u64,              // Block number
  pub nonce: u64,                     // Nonce : Nakamoto consensus
  pub coinbase: std::string::String,  // Founder of block reward
  pub transaction: Transaction,       // One transaction per block for now
  pub prev_hash: std::string::String, // Previous hash 00000 if genesis block
  pub hash: std::string::String,      // Block's hash
}