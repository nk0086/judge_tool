use anyhow::{anyhow, bail, ensure, Context, Result};
use colored::*;
use core::panic;
use dialoguer;
use reqwest;
use scraper::{element_ref::ElementRef, Html, Selector};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use tokio;
use url::Url;

pub async fn login() -> Result<()> {
    let mut map = HashMap::new();
    map.insert("username", "username");
    map.insert("password", "password");

    let url = "https://atcoder.jp/login";
    let client = reqwest::Client::new();
    let response = client.post(url).json(&map).send().await?;
    println!("{:#?}", response);

    let body = response.text().await?;
    println!("{:#?}", body);
    Ok(())
}
