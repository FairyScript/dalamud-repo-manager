use serde::Deserialize;

pub type JsonRepo = Vec<RepoItem>;

#[derive(Debug, Deserialize)]
pub struct RepoItem {
  Author: String,
  Name: String,
  Punchline: String,
  Description: String,
  InternalName: String,
  AssemblyVersion: String,
  RepoUrl: String,
  ApplicableVersion: String,
  DalamudApiLevel: i32,
  LoadPriority: i32,
  IconUrl: String,
  ImageUrls: Vec<String>,
  DownloadLinkInstall: String,
  IsHide: bool,
  IsTestingExclusive: bool,
  DownloadLinkTesting: String,
  DownloadLinkUpdate: String,
  DownloadCount: i32,
  LastUpdated: String,
}
