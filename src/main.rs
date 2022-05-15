/*
テストおよび提出の自動化
ーurlの取得
ーテストケースのダウンロード
ーテストの実行
ー提出
 */
mod get_testcase;
mod submit;
mod test_judge;
use crate::get_testcase::get_testcase;
use crate::submit::submit;
use crate::test_judge::test_judge;
use anyhow::{ensure, Context, Result};
use clap::Parser;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args = Arg::parse();
    //let mut path = env::current_dir()?;

    if let Some(file_name) = args.new {
        get_testcase(&file_name)?;
    } else if let Some(file_name) = args.test {
        test_judge(&file_name)?;
    } else if let Some(file_name) = args.submit {
        submit(&file_name)?;
    }

    Ok(())
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
