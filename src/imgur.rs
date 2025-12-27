use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, HOST, CONNECTION, ACCEPT_LANGUAGE, USER_AGENT,ORIGIN,REFERER};
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct Imgur {
    api: String,
    client_id: String,
    headers: Arc<Mutex<HeaderMap>>,
}

impl Imgur {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.imgur.v1+json"));
        headers.insert(ORIGIN, HeaderValue::from_static("https://imgur.com"));
        headers.insert(REFERER, HeaderValue::from_static("https://imgur.com/"));
        headers.insert(HOST, HeaderValue::from_static("api.imgur.com"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"));

        Self {
            api: "https://api.imgur.com".to_string(),
            client_id: "d70305e7c3ac5c6".to_string(),
            headers: Arc::new(Mutex::new(headers)),
        }
    }

    pub async fn get_welcome_message(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/homepage/v1/messages/random?client_id={}&filter[type]=welcome", self.api,self.client_id);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_tags(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/3/tags?client_id={}", self.api,self.client_id);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }



    pub async fn get_user_posts(&self, username: &str,include: &str,page: i32,sort: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/post/v1/accounts/{}?client_id={}&include={}&page={}&sort={}", self.api,username,self.client_id,include,page,sort);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_user_info(&self, username: &str,include: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/account/v1/accounts/{}?client_id={}&include={}", self.api,username,self.client_id,include);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_post_meta(&self, id: &str,include: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/post/v1/posts/{}/meta?client_id={}&include={}", self.api,id,self.client_id,include);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_post_info(&self, id: &str,include: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/post/v1/posts/{}?client_id={}&include={}", self.api,id,self.client_id,include);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn suggest(&self, inflate: &str,q: &str,types: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/3/suggest?client_id={}&inflate={}&q={}&types={}", self.api,self.client_id,inflate,q,types);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_comments_by_user(&self, filter: &str,include: &str,sort: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/comment/v1/comments?client_id={}&filter[account]={}&include={}&sort={}", self.api,self.client_id,filter,include,sort);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_comments(&self, filter: &str,include: &str,per_page: i32,sort: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/comment/v1/comments?client_id={}&filter[post]={}&include={}&per_page={}&sort={}", self.api,self.client_id,filter,include,per_page,sort);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn get_posts(&self, filter: &str,include: &str,location: &str,page: i32,sort: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/post/v1/posts?client_id={}&filter[section]={}&include={}&location={}&page={}&sort={}", self.api,self.client_id,filter,include,location,page,sort);
        let client = reqwest::Client::new();
        let current_headers = self.headers.lock().unwrap().clone();

        let response = client
            .get(&url)
            .headers(current_headers)
            .send()
            .await?;
        let body = response.json().await?;

        Ok(body)
    }
}
