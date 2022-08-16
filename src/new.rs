use crate::get_testcase::get_testcase;
use crate::json::read_json;
use anyhow::Result;
use easy_scraper::Pattern;
use std::collections::HashSet;
use std::env;
use std::fs;

//ファイル作成時は拡張子を指定
//テストの時は、拡張子から予測
pub fn new(contest_id: &str, extension: &str) -> Result<()> {
    let command_json = read_json();
    let extension = if extension == "default" {
        command_json["new"][extension].as_str().unwrap()
    } else {
        extension
    };

    fs::create_dir(contest_id)?;
    env::set_current_dir(contest_id)?;

    let url = format!("https://atcoder.jp/contests/{}/tasks", contest_id);
    let body = reqwest::blocking::get(url)?.text()?;

    let pat = format!(
        r#"
        <a href="/contests/{}/tasks/{{{}}}"></a>
        "#,
        contest_id, "{id}"
    );

    let problem_id = Pattern::new(&pat).unwrap();
    let problem_id = problem_id.matches(&body);
    let mut id_array = HashSet::new();
    for i in &problem_id {
        id_array.insert(i["id"].clone());
    }

    for id in id_array {
        get_testcase(&id)?;
        fs::File::create(id + "." + extension)?;
    }

    Ok(())
}
