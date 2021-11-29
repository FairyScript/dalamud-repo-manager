//get json
fn get_json() {
  let resp = reqwest::get("https://httpbin.org/ip")
    .await?
    .json::<HashMap<String, String>>()
    .await?;
  println!("{:#?}", resp);
  println!("{:#?}", resp["origin"]);
}

//parse toml
fn parse_toml() {
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

  fn_test();
  mod_test::mod_fn();
}
