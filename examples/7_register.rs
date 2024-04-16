use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
};

fn main() {
    let rpc_url = "http://localhost:8899";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    let payer = Keypair::read_from_file("id.json").unwrap();

    let ix = ore::instruction::register(payer.pubkey());

    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let result = rpc_client.send_transaction(&tx).unwrap();
    println!("result {:?}", result);
}
