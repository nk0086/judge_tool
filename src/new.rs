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
        fs::File::create(id + extensions)?;
    }

    Ok(())
}

//pub fn get_testcase(file_name: &str) -> Result<()> {
//    let mut path = env::current_dir()?;
//    path.push("test_cases");
//    path.push(file_name);
//    fs::create_dir_all(&path)?;
//    env::set_current_dir(&path)?;
//
//    //println!("{:?}", path);
//    let url = get_url(file_name);
//    let body = reqwest::blocking::get(url)?.text()?;
//
//    let input_case = Pattern::new(r#"<h3>入力例 {{num}}</h3><pre>{{test_case}}</pre>"#).unwrap();
//    let output_case = Pattern::new(r#"<h3>出力例 {{num}}</h3><pre>{{test_case}}</pre>"#).unwrap();
//
//    let input_case = input_case.matches(&body);
//    let output_case = output_case.matches(&body);
//    let number_of_test_case = input_case.len();
//
//    for i in 0..number_of_test_case {
//        let input_test_case = &input_case[i];
//        let output_test_case = &output_case[i];
//        //println!("入力{}: {:#?}", i + 1, input_test_case);
//        let mut in_case = fs::File::create("in_".to_string() + &(i + 1).to_string())?;
//        let mut out_case = fs::File::create("out_".to_string() + &(i + 1).to_string())?;
//        in_case.write_all(input_test_case["test_case"].as_bytes())?;
//        out_case.write_all(output_test_case["test_case"].as_bytes())?;
//    }
//    Ok(())
//}
