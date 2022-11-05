use anyhow::Result;
use easy_scraper::Pattern;
use reqwest;

// login to atcoder
pub async fn login_to_atcoder() -> Result<()> {
    let url = "https://atcoder.jp/login";
    // input your username and password
    let mut username = String::new();
    let mut password = String::new();

    username.push_str("pori_na");
    password.push_str("Hjcg1872");
    //println!("input your username");
    //std::io::stdin().read_line(&mut username)?;
    //println!("input your password");
    //std::io::stdin().read_line(&mut password)?;

    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.text().await?;
    let pat = Pattern::new(
        r#"
        <input type="hidden" name="csrf_token" value="{{token}}">
        "#,
    )
    .unwrap();

    let csrf_token = pat.matches(&body)[0]["token"].to_string();
    let url = format!(
        "https://atcoder.jp/login?username={}&password={}&csrf_token={}",
        username, password, csrf_token
    );
    let res = client.post(&url).send().await?;
    println!("{:?}", res);
    //let res = client
    //    .post(url)
    //    .form(&[
    //        ("username", username.trim()),
    //       "password", password.trim()),
    //        ("csrf_token", &csrf_token),
    //    ])
    //    .send()
    //    .await?;

    // set cookie
    let cookie = res.headers().get("set-cookie").unwrap().to_str().unwrap();
    let cookie = cookie.split(";").collect::<Vec<&str>>()[0];
    println!("{}", cookie);

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
