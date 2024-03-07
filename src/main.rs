use anyhow::{Context, Result};
use reqwest::Response;

async fn fetch() -> Result<Response, reqwest::Error> {
    let url = "https://metroverse.com/blocks/14623";
    // let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .tls_built_in_root_certs(true)
        .build()
        .unwrap();
    let result = client.get(url).send().await;
    println!("{result:?}");
    result
}

#[tokio::main]
async fn main() -> Result<()> {
    fetch().await.context("failed fetch")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_block() {
        assert!(fetch().await.is_ok())
    }
}
