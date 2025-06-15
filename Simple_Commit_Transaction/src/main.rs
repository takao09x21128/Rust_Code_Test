use ethers::prelude::*;
use std::convert::TryFrom;
use eyre::Result;
use std::env;

fn usage() {
    println!("
        Simple_Commit_Transaction <rpc_url> <transaction_hex>

        rpc_url         https://xxxx
        transactin_hex  0xabcc.....
    ");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        usage();
        return Ok(());
    }

    let provider = Provider::<Http>::try_from(args[0].clone());
    let binding = provider.expect("Commit");

    let pending_tx = binding.send_raw_transaction(args[1].parse()?).await?;

    let tx_hash : H256 = *pending_tx;
    println!("transaction sent! Has: {:#x}", tx_hash);

    let receipt = pending_tx.await?;

    match receipt {
        Some(r) => {
            println!("Transaction confirmed in block: {}", r.block_number.unwrap());
            println!("Status: {}", if r.status == Some(U64::from(1)) { "Success" } else { "Failed" });
        }
        None => {
            println!("Transaction dropped or not mined.");
        }
    }

    Ok(())
}