use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
};

fn main() {
    let rpc_url = "http://localhost:8899";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    let payer = Keypair::read_from_file("id.json").unwrap();

    // We need the 32 bit secret key, not the 64 bit keypair
    let secret_key = payer.secret();
    println!("secret_key {:?}, length {}", secret_key.as_bytes(), secret_key.as_bytes().len());

    let ix = ore::instruction::initialize(payer.pubkey());

    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let result = rpc_client.send_transaction(&tx).unwrap();
    println!("result {:?}", result);
}
