use hyper_tls::HttpsConnector;
use hyper::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://api.ipify.org?format=json".parse::<hyper::Uri>().unwrap();
    let res = fetch_url(url).await?;
    println!("{}", res);
    Ok(())
}

async fn fetch_url(url: hyper::Uri) -> Result<String> {
    let mut result = String::new();
    let https = HttpsConnector::new().expect("https connector works");
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    // println!("Response: {}", res.status());
    // println!("Headers: {:#?}\n", res.headers());

    let mut body = res.into_body();

    while let Some(next) = body.next().await {
        let chunk = next?;
        let bytes = String::from_utf8_lossy(&chunk);
        result.push_str(&bytes);
    }

    println!("\n\nDone!");

    Ok(result)
}
