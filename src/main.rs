use std::{collections::HashMap, error::Error};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let response = client.get("https://httpbin.org/ip")
    .send();
    let resp = match response.await {
        Ok(it) => it.json::<HashMap<String, String>>().await?,
        Err(err) => panic!("请求出错，{:?}", err),
    };
    // let response = reqwest::get("https://httpbin.org/ip")
    // .await?
    // .json::<HashMap<String, String>>()
    // .await?;
    println!("{:?}", resp.get("origin").unwrap());
    Ok(())
}
