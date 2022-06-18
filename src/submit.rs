use anyhow::{Context, Result};
use scraper::{element_ref::ElementRef, Html, Selector};
use tokio;

#[tokio::main]
pub async fn submit(file_name: &str) -> Result<()> {
    let url = format!("https://atcoder.jp/contests/{}/submit", file_name);

    let client = reqwest::Client::new();
    let doc = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let doc = Html::parse_document(&doc);
    let csrf_token = doc
        .select(&Selector::parse("input[name=\"csrf_token\"]").unwrap())
        .next()
        .with_context(|| "cannot find csrf_token")?;

    let csrf_token = csrf_token
        .value()
        .attr("value")
        .with_context(|| "cannot find csrf_token")?;

    //println!("{:?}", doc);

    //let url = url.to_string() + "/login";

    Ok(())
}
