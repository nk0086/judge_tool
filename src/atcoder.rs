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

// login to atcoder
pub fn login_to_atcoder() {
    let mut username = String::new();
    let mut password = String::new();
    let mut session = String::new();
    let mut csrf_token = String::new();
    let mut cookie = String::new();
    let mut login = false;

    // get username and password
    println!("Please enter your username and password.");
    println!("username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("password: ");
    std::io::stdin().read_line(&mut password).unwrap();

    // get csrf_token
    let url = "https://atcoder.jp/login";
    let res = reqwest::blocking::get(url).unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("meta[name=csrf-token]").unwrap();
    for element in document.select(&selector) {
        csrf_token = element.value().attr("content").unwrap().to_string();
    }

    // login
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("X-CSRF-Token", csrf_token)
        .body(format!(
            "username={}&password={}",
            username.trim(),
            password.trim()
        ))
        .send()
        .unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("div.alert-danger").unwrap();
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<Vec<_>>()[0]);
        login = false;
    }
    if login == false {
        println!("login failed");
        std::process::exit(1);
    }

    // get session
    let url = "https://atcoder.jp/";
    let res = reqwest::blocking::get(url).unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("meta[name=csrf-token]").unwrap();
    for element in document.select(&selector) {
        session = element.value().attr("content").unwrap().to_string();
    }

    // get cookie
    let url = "https://atcoder.jp/";
    let res = reqwest::blocking::get(url).unwrap();
    let headers = res.headers();
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    let cookie: Vec<_> = cookie.split(";").collect();
    let cookie = cookie[0].to_string();

    // write cookie to file
    let mut file = File::create("cookie.txt").unwrap();
    file.write_all(cookie.as_bytes()).unwrap();
}

//pub async fn login() -> Result<()> {
//    let mut map = HashMap::new();
//    map.insert("username", "username");
//    map.insert("password", "password");
//
//    let url = "https://atcoder.jp/login";
//    let client = reqwest::Client::new();
//    let response = client.post(url).json(&map).send().await?;
//    println!("{:#?}", response);
//
//    let body = response.text().await?;
//    println!("{:#?}", body);
//    Ok(())
//}
