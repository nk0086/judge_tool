use anyhow::{Context, Result};
use easy_scraper::Pattern;

pub fn get_testcase(file_name: &str) -> Result<()> {
    let url = get_url(file_name);
    let body = reqwest::blocking::get(url)?.text()?;

    let pattern = Pattern::new(
        r#"
    <section>
        <pre>{{test_case}}</pre>
    </section>
        "#,
    )
    .unwrap();

    println!("{:#?}", pattern.matches(&body));

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
