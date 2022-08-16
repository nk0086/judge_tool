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
    version = "0.0.9",
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
    extension: Option<String>,
    #[clap(short, long)]
    login: Option<String>,
    #[clap(short, long)]
    command: Option<String>,
}

fn main() -> Result<()> {
    let args = Arg::parse();

    let extension = if let Some(k) = args.extension {
        format!("{}", &k)
    } else {
        format!("default")
    };

    // json で指定したデフォルトのファイルを作成できるようにする
    if let Some(contest_name) = args.new {
        // ex) contest_name: abc250
        new(&contest_name, &extension)?;
    } else if let Some(file_name) = args.test {
        // ex) file_name: abc250_a.py
        let file_name: Vec<_> = file_name.split(".").collect();

        get_testcase(&file_name[0])?;
        test_judge(&file_name[0], &file_name[1])?;
    } else if let Some(problem_name) = args.submit {
        submit(&problem_name)?;
    }

    Ok(())
}
