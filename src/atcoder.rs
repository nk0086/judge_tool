use anyhow::{anyhow, bail, ensure, Context, Result};
use colored::*;
use core::panic;
use dialoguer;
use reqwest;
use scraper::{element_ref::ElementRef, Html, Selector};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use tokio;
use url::Url;

pub async fn login(user_name: &str, password: &str) -> Result<()> {
    let url = "https://atcoder.jp/login";

    let client = reqwest::Client::new();
    let doc = client
        .get(url)
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

    //let user_name = dialoguer::Input::<String>::new()
    //    .with_prompt("user_name")
    //    .interact()?;

    //let password = dialoguer::Password::new()
    //    .with_prompt("password")
    //    .interact()?;

    let res = client
        .post(url)
        .form(&[
            ("username", user_name),
            ("password", password),
            ("csrf_token", csrf_token),
        ])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let res = Html::parse_document(&res);
    if let Some(err) = res
        .select(&Selector::parse("div.alert-danger").unwrap())
        .next()
    {
        bail!(
            "Login failed: {}",
            err.last_child().unwrap().value().as_text().unwrap().trim()
        );
    }

    if res
        .select(&Selector::parse("div.alert-success").unwrap())
        .next()
        .is_some()
    {
        return Ok(());
    }

    Err(anyhow!("Login failed: Unknown error"))
}
