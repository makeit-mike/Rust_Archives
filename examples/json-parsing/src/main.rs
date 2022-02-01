extern crate serde_json;
extern crate reqwest;
extern crate futures;

#[derive(serde::Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

async fn parseJson() -> Result<(), reqwest::Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "makeit-mike",
                              repo = "rust_archives");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}

fn main() {
    let res = parseJson();
    futures::executor::block_on(res);
}

fn main2() {
    let json_str = r#"
        {
            "name":"michael",
            "age": 24,
            "is_male": true
        }
    "#;
    let json_from_url = "https://randomuser.me/api/";

    let result = serde_json::from_str(json_str);

    if result.is_ok() {
        let parsed: serde_json::Value = result.unwrap();
        println!("Name: {}", parsed["name"].as_str().unwrap());
    } else {
        println!("Failed to parse JSON.");
    }
}
