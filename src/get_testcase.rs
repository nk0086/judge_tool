use anyhow::{Context, Result};
use easy_scraper::Pattern;
use std::env;
use std::fs;
use std::io::prelude::*;

pub fn get_testcase(file_name: &str) -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("test_cases");
    path.push(file_name);
    //fs::create_dir(&path)?;
    env::set_current_dir(&path).unwrap();

    //println!("{:?}", path);
    let url = get_url(file_name);
    let body = reqwest::blocking::get(url)?.text()?;

    let input_case = Pattern::new(r#"<h3>入力例 {{num}}</h3><pre>{{test_case}}</pre>"#).unwrap();
    let output_case = Pattern::new(r#"<h3>出力例 {{num}}</h3><pre>{{test_case}}</pre>"#).unwrap();

    let input_case = input_case.matches(&body);
    let output_case = output_case.matches(&body);
    let number_of_test_case = input_case.len();

    for i in 0..number_of_test_case {
        let input_test_case = &input_case[i];
        let output_test_case = &output_case[i];
        //println!("入力{}: {:#?}", i + 1, input_test_case);
        let mut in_case = fs::File::create("in_".to_string() + &(i + 1).to_string())?;
        let mut out_case = fs::File::create("out_".to_string() + &(i + 1).to_string())?;
        in_case.write_all(input_test_case["test_case"].as_bytes())?;
        out_case.write_all(output_test_case["test_case"].as_bytes())?;
    }
    Ok(())
}

//todo: abcxxx_d 以外の形式の時の処理
fn get_url(file_name: &str) -> String {
    let contest_name: Vec<&str> = file_name.split("_").collect();
    let base_url =
        "https://atcoder.jp/contests/".to_string() + contest_name[0] + "/tasks/" + file_name;
    println!("{}", base_url);
    base_url
}

#[cfg(test)]
mod tests {
    use super::get_url;

    #[test]
    fn check_url() {
        assert_eq!(
            get_url("abc250_d"),
            "https://atcoder.jp/contests/abc250/tasks/abc250_d".to_string()
        );
    }
}
