mod utils;
mod types;

use reqwest::header::HeaderMap;
use utils::raxios::RaxiosClient;

use types::files::File;

const FIGMA_API_URL: &'static str = "https://api.figma.com/v1";

#[allow(dead_code)]
#[derive(Debug)]
pub struct Figma {
  access_token: String,
  client: RaxiosClient,
}

impl Figma {
  pub fn new(access_token: String) -> Self {
    let mut headers = HeaderMap::new();
    headers.insert("X-FIGMA-TOKEN", access_token.parse().unwrap());

    Self {
      access_token,
      client: RaxiosClient::new(Some(headers))
    }
  }

  pub async fn get_file(&self, file_id: &str) -> Option<String> {
    let url = format!("{}/files/{}", FIGMA_API_URL, file_id);
    let response = self.client.get(&url, None).await.expect("Failed to get file. Please check configuration");
    
    match response.status() {
      reqwest::StatusCode::OK => {
        println!("File fetched successfully");
      }

      reqwest::StatusCode::NOT_FOUND => {
        println!("Failed to get file. Invalid file_id passed");
        return None;
      }

      reqwest::StatusCode::FORBIDDEN => {
        panic!("Failed to get file. Invalid access token passed");
      }

      _ => {
        panic!("Failed to get file. Something went wrong!");
      }
    }

    let data = response.json::<File>().await;

    println!("{:?}", data);
    Some("".to_string())
  }
}