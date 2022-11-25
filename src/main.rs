use anyhow::Result;
use bitcoin::Address;
use electrum_client::{Client, ElectrumApi};

fn main() -> Result<()> {
    let client = Client::new("tcp://YOUR ELECTRUM SERVER")?;
    let addr = "YOUR ADDRESS HERE";

    let address: Address = addr.parse()?;
    println!("fetching history for {}", addr.to_string());
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
