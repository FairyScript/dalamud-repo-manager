use futures::{executor::ThreadPool, future::join_all};
use std::{
  error::Error,
  sync::{Arc, RwLock},
};
mod lib;
mod modal;
use lib::{parse_config, Config};
use modal::json_repo::{JsonRepo, RepoItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  //read config file
  let config = parse_config("profile/test.toml").expect("faild to load config");

  //let mut repos: JsonRepo = Vec::<RepoItem>::new();

  let pool = ThreadPool::new().expect("Failed to create threadpool");

  //pool.spawn_ok(repo_updater(config, &mut repos));

  let repos = Arc::new(RwLock::new(Vec::<RepoItem>::new()));

  let repos_clone = repos.clone();
  pool.spawn_ok(repo_updater(config, repos_clone));

  Ok(())
}

async fn repo_updater(config: Config, repos: Arc<RwLock<Vec<RepoItem>>>) {}

async fn web_server(config: Config, repos: Arc<RwLock<Vec<RepoItem>>>) {}

async fn get_repo(config: Config) -> Result<(), Box<dyn Error>> {
  let pool = config
    .repo
    .iter()
    .map(|item| get_repo_item(item.url.as_str()));

  join_all(pool).await;
  Ok(())
}

async fn get_repo_item(url: &str) -> Result<JsonRepo, Box<dyn Error>> {
  let resp: JsonRepo = reqwest::get(url).await?.json::<JsonRepo>().await?;
  println!("{:#?}", resp);

  Ok(resp)
}
