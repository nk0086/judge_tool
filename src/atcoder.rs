use anyhow::Result;
use easy_scraper::Pattern;
use reqwest;
use std::env;
use std::fs;
use std::io::prelude::*;

// login to atcoder
pub async fn login_to_atcoder() -> Result<()> {
    let url = "https://atcoder.jp/login";
    // input your username and password
    let mut username = String::new();
    let mut password = String::new();
    println!("input your username");
    std::io::stdin().read_line(&mut username)?;
    println!("input your password");
    std::io::stdin().read_line(&mut password)?;

    let body = reqwest::get(url).await?.text().await?;
    let pat = Pattern::new(
        r#"
        <input type="hidden" name="csrf_token" value="{{token}}">
        "#,
    )
    .unwrap();
    let csrf_token = pat.matches(&body)[0]["token"].to_string();

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .form(&[
            ("username", username.trim()),
            ("password", password.trim()),
            ("csrf_token", &csrf_token),
        ])
        .send()
        .await?;
    println!("{:?}", res);

    Ok(())
}
