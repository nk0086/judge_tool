use anyhow::Result;
use easy_scraper::Pattern;
use std::collections::HashSet;
use std::env;
use std::fs;

//ファイル作成時は拡張子を指定
//テストの時は、拡張子から予測
pub fn new(contest_id: &str, extensions: &str) -> Result<()> {
    fs::create_dir(contest_id)?;
    env::set_current_dir(contest_id)?;

    let url = format!("https://atcoder.jp/contests/{}/tasks", contest_id);
    let body = reqwest::blocking::get(url)?.text()?;

    //let pat = format!(
    //    r#"
    //    <a href="/contests/{}/submit?taskScreenName={{{}}}"></a>
    //    "#,
    //    contest_id, "{id}"
    //);
    //println!("{}", pat);
    let pat = format!(
        r#"
        <a href="/contests/abc250/tasks/{{{}}}"></a>
        "#,
        "{id}"
    );

    let problem_id = Pattern::new(&pat).unwrap();
    let problem_id = problem_id.matches(&body);
    let mut id_array = HashSet::new();
    for i in &problem_id {
        id_array.insert(i["id"].clone());
    }

    for id in id_array {
        fs::File::create(id + "." + extensions)?;
    }

    Ok(())
}
