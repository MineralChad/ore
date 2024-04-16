use std::str::FromStr;

use solana_program::keccak::Hash as KeccakHash;
use solana_sdk::{keccak::{hashv, HASH_BYTES}, program_memory::sol_memcmp, pubkey::Pubkey};
fn main() {
  let signer = Pubkey::from_str("CraELXiQqxtYsTNRW1NS5QTGc5JpypMayhRhCnLtTWRq").unwrap();

  let hash = KeccakHash::new_from_array([78, 0, 223, 163, 59, 117, 186, 207, 80, 200, 208, 86, 42, 148, 170, 26, 236, 206, 64, 126, 247, 33, 112, 72, 95, 142, 211, 185, 46, 78, 101, 189]);
  let difficulty = KeccakHash::new_from_array([0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);

  let next = find_next_hash(hash, difficulty, signer);

  println!("next_hash {:?}, nonce {}", next.0.0, next.1);
}

fn find_next_hash(hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) -> (KeccakHash, u64) {
  let mut next_hash: KeccakHash;
  let mut nonce = 0u64;
  loop {
      next_hash = hashv(&[
          hash.to_bytes().as_slice(),
          signer.to_bytes().as_slice(),
          nonce.to_le_bytes().as_slice(),
      ]);
      if next_hash.le(&difficulty) {
          break;
      } else {
          // println!("Invalid hash: {} Nonce: {:?}", next_hash.to_string(), nonce);
      }

      nonce += 1;
  }
  (next_hash, nonce)
}
