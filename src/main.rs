/*
テストおよび提出の自動化
ーurlの取得
ーテストケースのダウンロード
ーテストの実行
ー提出
 */
use anyhow::{ensure, Context, Result};
use clap::Parser;

use crate::get_testcase::get_testcase;

mod get_testcase;

fn main() {
    let args = Arg::parse();
    get_testcase(&args.test.unwrap());
    //println!("{:?}", args);
}

#[derive(Parser, Debug)]
#[clap(
    name = "auto judge tools",
    version = "1.0.0",
    author = "nk0086",
    about = "Support for testing and submitting code in AtCoder."
)]
struct Arg {
    ///select option: make file, test code, submit code
    #[clap(short, long)]
    test: Option<String>,
    #[clap(short, long)]
    submit: Option<String>,
    #[clap(short, long)]
    new: Option<String>,
}
