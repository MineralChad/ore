use ore::BUS_ADDRESSES;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
};
use solana_program::keccak::Hash as KeccakHash;

fn main() {
    // let rpc_url = "http://localhost:8899";
    // let rpc_client = RpcClient::new(rpc_url.to_string());

    let payer = Keypair::read_from_file("id.json").unwrap();

    let next_hash = KeccakHash::new_from_array([0, 0, 0, 116, 80, 36, 177, 16, 251, 48, 48, 52, 161, 239, 249, 79, 42, 37, 104, 250, 149, 114, 177, 120, 125, 162, 117, 74, 15, 68, 124, 13]);
    let nonce = 11495037;

    let ix = ore::instruction::mine(payer.pubkey(), BUS_ADDRESSES[0], ore::state::Hash(next_hash.to_bytes()), nonce);

    println!("ix data {:?}", ix.data);

    // let blockhash = rpc_client.get_latest_blockhash().unwrap();
    // let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    // let result = rpc_client.send_transaction(&tx).unwrap();
    // println!("result {:?}", result);
}
