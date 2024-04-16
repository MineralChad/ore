use std::str::FromStr;

use solana_program::keccak::Hash as KeccakHash;
use solana_sdk::{keccak::{hashv, HASH_BYTES}, program_memory::sol_memcmp, pubkey::Pubkey};
fn main() {
  let signer = Pubkey::from_str("CraELXiQqxtYsTNRW1NS5QTGc5JpypMayhRhCnLtTWRq").unwrap();

  let hash = KeccakHash::new_from_array([78, 0, 223, 163, 59, 117, 186, 207, 80, 200, 208, 86, 42, 148, 170, 26, 236, 206, 64, 126, 247, 33, 112, 72, 95, 142, 211, 185, 46, 78, 101, 189]);
  let difficulty = KeccakHash::new_from_array([0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);

  let next_hash = KeccakHash::new_from_array([0, 0, 0, 116, 80, 36, 177, 16, 251, 48, 48, 52, 161, 239, 249, 79, 42, 37, 104, 250, 149, 114, 177, 120, 125, 162, 117, 74, 15, 68, 124, 13]);
  let nonce = 11495037;

  let valid = validate_hash(next_hash, hash, signer, nonce, difficulty);
  println!("valid {}", valid);
}

fn validate_hash(
  hash: KeccakHash,
  current_hash: KeccakHash,
  signer: Pubkey,
  nonce: u64,
  difficulty: KeccakHash,
) -> bool {
  // Validate hash correctness
  let hash_ = hashv(&[
      current_hash.as_ref(),
      signer.as_ref(),
      nonce.to_le_bytes().as_slice(),
  ]);
  if sol_memcmp(hash.as_ref(), hash_.as_ref(), HASH_BYTES) != 0 {
      return false;
  }

  // Validate hash difficulty
  if hash.gt(&difficulty) {
    return false;
  }

  true
}
