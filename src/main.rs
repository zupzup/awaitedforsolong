extern crate tokio;
extern crate hyper;

use std::env;
use std::io::{self, Write};
use hyper_tls::HttpsConnector;
use hyper::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => "https://api.ipify.org?format=json".to_owned()
    };

    let url = url.parse::<hyper::Uri>().unwrap();
    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let https = HttpsConnector::new().expect("https connector works");
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let mut body = res.into_body();

    while let Some(next) = body.next().await {
        let chunk = next?;
        io::stdout().write_all(&chunk)?;
    }

    println!("\n\nDone!");

    Ok(())
}
