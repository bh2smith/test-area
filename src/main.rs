use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://metroverse.com/blocks/14623";
    let response = reqwest::Client::new().get(url).send().await;
    println!("Reqwest {response:?}");

    let client = reqwest::Client::builder()
        .tls_built_in_root_certs(true)
        .build()
        .unwrap();
    let x = client.get(url).send().await;
    println!("Reqwest {x:?}");
    Ok(())
}
