
mod ledgerx;
mod sheeldmarket;
use futures::executor::block_on;
use serde_json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let contracts = block_on(ledgerx::get_contracts());

    match contracts{
        Some(ds) =>
            for d in ds.data.iter() {
                println!("{:#?}", d);
            },
        None => println!("Couldn't get Contracts")

    }

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

