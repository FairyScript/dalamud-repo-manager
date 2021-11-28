use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Item {
    foo: String,
    bar: String,
}

type Entry = HashMap<String, Vec<Item>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    println!("{:#?}", resp["origin"]);

    let toml_string = r#"
  [[entry]]
  foo = "a0"
  bar = "b0"

  [[entry]]
  foo = "a1"
  bar = "b1"
  "#;
    let config: Entry = toml::from_str(toml_string).unwrap();
    println!("{:#?}", config);

    Ok(())
}
