/*
テストおよび提出の自動化
ーurlの取得
ーテストケースのダウンロード
ーテストの実行
ー提出
 */
mod atcoder;
mod get_testcase;
mod json;
mod new;
mod submit;
mod test_judge;
use crate::get_testcase::get_testcase;
use crate::json::read_json;
use crate::new::new;
use crate::submit::submit;
use crate::test_judge::test_judge;

use anyhow::Result;
use clap::Parser;

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
    #[clap(short, long)]
    extensions: Option<String>,
    #[clap(short, long)]
    login: Option<String>,
    #[clap(short, long)]
    command: Option<String>,
}

fn main() -> Result<()> {
    let args = Arg::parse();

    let extensions = if let Some(k) = args.extensions {
        format!("{}", &k)
    } else {
        format!("default")
    };

    if let Some(file_name) = args.new {
        new(&file_name, &extensions)?;
    } else if let Some(file_name) = args.test {
        get_testcase(&file_name)?;
        test_judge(&file_name, &extensions)?;
    } else if let Some(file_name) = args.submit {
        submit(&file_name)?;
    }

    Ok(())
}
