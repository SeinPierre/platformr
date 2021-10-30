
mod ledgerx;
mod sheeldmarket;
use ledgerx::Contract;
use futures::executor::block_on;
use serde_json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut contracts_maps : HashMap<String,&Contract> = HashMap::new();
    let mut prices_map : HashMap<String,f64> = HashMap::new();

    // Load Contracts into the map
    let contracts = block_on(ledgerx::get_contracts());

    match contracts{
        Some(ds) =>
            for d in ds.data.iter() {
                println!("{:#?} : {:#?} ", d.label, d.strike_price);
                contracts_maps.insert(d.label.clone(), d);
            },
        None => println!("Couldn't get Contracts")

    }

    // Load prices into the map
    let prices = block_on(sheeldmarket::get_prices());
    let prices_text :String;

    match prices{
        Err(_) => println!("Error collecting Prices"),
        Ok(p) => {
            prices_text = p;
            let prices: HashMap<String,f64> = serde_json::from_str(&prices_text)?;
            println!("hash map = {:#?}", prices);
        }
    }


    Ok(())
}

