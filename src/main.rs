use anyhow::Result;
use fantoccini::{ClientBuilder, Client, Locator};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // Initial navigation
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // Test client1 function
    client1(&c, "https://en.wikipedia.org/wiki/Main_Page").await?;

    // Test client2 function  
    client2(&c, "https://en.wikipedia.org/wiki/Flower").await?;

    c.close().await?;
    Ok(())
}

pub async fn client1(c: &Client, url1: &str) -> Result<(), fantoccini::error::CmdError> {
    c.goto(url1).await?;
    println!("Navigated to: {}", url1);
    let current_url = c.current_url().await?;
    println!("Current URL: {}", current_url);
    Ok(())
}

pub async fn client2(c: &Client, url2: &str) -> Result<(), fantoccini::error::CmdError> {
    c.goto(url2).await?;
    println!("Navigated to: {}", url2);
    let current_url = c.current_url().await?;
    println!("Current URL: {}", current_url);
    Ok(())
}
