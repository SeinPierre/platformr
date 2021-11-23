pub async fn get_prices() -> Result<String,reqwest::Error>{

    let res = reqwest::Client::new()
        .get("https://markets.sheeldmarket.com/prices")
        .send()
        .await?;


    let t = res
        .text()
        .await?;

    Ok(t)
}