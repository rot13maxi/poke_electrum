use anyhow::Result;
use bitcoin::Address;
use clap::Parser;
use electrum_client::{Client, ElectrumApi};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Fetch transaction history for a given address from an electrum server")]
struct Args {
    electrum_server: String,
    address: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new(&format!("tcp://{}", &args.electrum_server))?;

    let address: Address = args.address.parse()?;
    println!("fetching history for {}", address.to_string());
    println!("tx-id, height, direction, amount sent/received");
    let history = client.script_get_history(&address.script_pubkey())?;
    history.iter().for_each(|ghr| {
        let height = ghr.height;
        let tx = client.transaction_get(&ghr.tx_hash).unwrap();
        let inbound = tx.output.iter()
            .map(|txout| Address::from_script(&txout.script_pubkey, bitcoin::Network::Bitcoin).unwrap())
            .any(|a| a == address);
        let amount: u64 = match inbound {
            true => tx.output.iter().find(|item| item.script_pubkey == address.script_pubkey()).unwrap().value,
            false =>  tx.output.iter().fold(0 as u64, |x, y| x + y.value)
        };
        println!("{}, {}, {}, {}", &ghr.tx_hash, height, if inbound {"receive"} else {"send"}, amount);
    });
    Ok(())
}
