use anyhow::Result;
use easy_scraper::Pattern;
use reqwest;

const ATCODER_URL: &str = "https://atcoder.jp";
// login to atcoder
pub async fn login_to_atcoder() -> Result<()> {
    let url = format!("{}/login", ATCODER_URL);
    // input your username and password
    let mut username = String::new();
    let mut password = String::new();

    //println!("input your username");
    //std::io::stdin().read_line(&mut username)?;
    //println!("input your password");
    //std::io::stdin().read_line(&mut password)?;

    //open test.html in local file
    let html = std::fs::read_to_string("src/test.html")?;
    let pat = Pattern::new(
        r#"
        <div id="select-lang-abc275_h">
        <select class="form-control">
        <option value="{{num}}">{{lang}}</option>
        </select>
        </div>
        "#,
    )
    .unwrap();

    let matches = pat.matches(&html);
    for m in matches {
        println!("{:?}", m);
    }

    Ok(())
}
