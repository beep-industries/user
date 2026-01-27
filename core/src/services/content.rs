use serde::{Deserialize, Serialize};

pub trait ContentServiceClient: Send + Sync + Clone {
    fn get_profile_picture_url(&self, url: &str) -> impl Future<Output = Result<String, String>> + Send;
}


#[derive(Clone)]
pub struct ContentServiceClientImpl {
    client: reqwest::Client,
    base_url: String,
}

impl ContentServiceClientImpl {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ContentSigningPayload {
    #[serde(rename = "action")]
    action: String,
    #[serde(rename = "expires_in_ms")]
    expires_in: u64,
}

impl ContentServiceClient for ContentServiceClientImpl {
    async fn get_profile_picture_url(&self, user_id: &str) -> Result<String, String> {
        let url = format!("{}/profile_picture/{}", self.base_url, user_id);
        let payload = ContentSigningPayload {
            action: "Put".to_string(),
            expires_in: 60 * 60 * 24 * 7,
        };
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to get profile picture: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()));
        }

        Ok(response.url().to_string())
    }
}

