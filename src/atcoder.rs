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

    println!("input your username");
    std::io::stdin().read_line(&mut username)?;
    println!("input your password");
    std::io::stdin().read_line(&mut password)?;

    let client = reqwest::Client::new();
    let body = client.get(&url).send().await?.text().await?;
    // set cookie
    let cookie = client
        .get(&url)
        .send()
        .await?
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let cookie = cookie.split(";").collect::<Vec<&str>>()[0];
    // restore cookie
    let cookie = format!(
        "{};{}",
        cookie,
        body.split("name=\"csrf_token\"").collect::<Vec<&str>>()[1]
            .split("value=\"")
            .collect::<Vec<&str>>()[1]
            .split("\"")
            .collect::<Vec<&str>>()[0]
    );

    let pat = Pattern::new(
        r#"
        <input type="hidden" name="csrf_token" value="{{token}}" />
        "#,
    )
    .unwrap();

    let csrf_token = pat.matches(&body)[0]["token"].to_string();
    println!("csrf_token: {}", csrf_token);
    // ログイン認証 クエリパラメータ
    let params = [
        ("username", username.trim()),
        ("password", password.trim()),
        ("csrf_token", &cookie),
    ];
    let res = client.post(&url).form(&params).send().await?;
    let body = res.text().await?;
    let pat = Pattern::new(
        r#"
        <span class="user-{{color}}">{{name}}</span>
        "#,
    )
    .unwrap();

    dbg!(&body);

    let user_name = pat.matches(&body)[1]["name"].to_string();
    if user_name == username.trim() {
        println!("login success");
        println!("{}", body);
    } else {
        println!("login failed");
    }

    //let pat = Pattern::new(
    //    r#"
    //    <div class="alert">
    //    {{error_message}}
    //    </div>
    //    "#,
    //)
    //.unwrap();

    Ok(())
}
