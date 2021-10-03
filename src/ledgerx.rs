use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    id: i32,
    label: String,
    name: Option<String>,
    is_call: bool,
    active: bool,
    strike_price: i32,
    min_increment: i32,
    date_live: String,
    date_expires: String,
    date_exercise: String,
    underlying_asset: String,
    collateral_asset: String,
    derivative_type: String,
    open_interest: Option<i32>,
    is_next_day: bool,
    multiplier: i32,
    _type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    total_count: i32,
    next: Option<String>,
    previous: Option<String>,
    limit: i32,
    offset: i32,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub data: Vec<Contract>,
    pub meta: Meta,
}

pub async fn get_contracts() -> Option<Answer> {
    
    let res = reqwest::Client::new()
        .get("https://api.ledgerx.com/trading/contracts")
        .query(&[("active", "true")])
        .query(&[("derivative_type", "options_contract")])
        .send()
        .await.ok()?;

    let t = res
        .text()
        .await.ok()?
        .replace("type","_type")
        .replace("derivative__type","derivative_type");

    // let y: String = t.chars().skip(69088).take(20).collect();

    // println!("{}", y);

    serde_json::from_str(&t).ok()?

}