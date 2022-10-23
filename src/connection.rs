use std::collections::HashMap;
use serde::Deserialize;
use reqwest::Client;
use tokio_compat_02::FutureExt;

pub struct Connection {
    client: Client,
    server_url: String,
}

#[derive(Deserialize)]
pub struct Mice {
    pub mice: HashMap<String, [f64; 2]>
}

impl Connection {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            server_url: url,
        }
    }

    pub async fn _get_json(&self, sub_url: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let resp = reqwest::get(format!("{}{}", self.server_url,sub_url))
        .await?
        .json::<HashMap<String, String>>()
        .await?;
        Ok(resp)
    }

    pub async fn get_mice(&self) -> Result<Mice, Box<dyn std::error::Error>> {
        let resp = self.client.get(format!("{}/mice", self.server_url)).send()
        .await?;
        /*.json()
        .await?;*/
        // Ok(resp)
        Ok(Mice {
            mice: HashMap::new(),
        })
    }

    pub async fn send_mouse(&self, mouse: [f64; 2]) {
        // let mut params = HashMap::new();
        // params.insert("pos", mouse);    
        // println!("{:?}", params);
        // let _resp = self.client.post(format!("{}/mice/upload", self.server_url)).form(&params).send();
    }
}