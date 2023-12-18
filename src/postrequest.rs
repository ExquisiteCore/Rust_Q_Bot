use serde_json::{Map, Value, json};
use std::collections::HashMap;
pub struct Request {
    http_server_url: String,
    client: reqwest::Client,
}
impl Request {
        //post请求
    async fn send_post_request(&self, api: &str,json_data: &str) ->Result<String, reqwest::Error> {
    // 发送 HTTP POST 请求
    match self.client.post(format!("{}{}", self.http_server_url, api))
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // 成功获取响应
                match response.text().await {
                    Ok(text) => {
                        Ok(text)
                    }
                    Err(err) => {
                        eprintln!("Error reading response text: {:?}", err);
                        Err(err)
                    }
                }
            } else {
                eprintln!("HTTP request failed with status code: {}", response.status());
                Err(response.error_for_status().unwrap_err())
            }
        }
        Err(err) => {
            eprintln!("Error sending HTTP request: {:?}", err);
            Err(err)
        }
    }
    }
    pub async fn new(server_url: &str,httpport:i32) -> Self {
        let http_server_url = format!("http://{}:{}/", server_url, httpport);
        Request {
            http_server_url,
            client: reqwest::Client::new(),
        }
    }
}

impl Request {//高级封装
    pub async fn send_private_msg_api(&self) -> Result<String, reqwest::Error> {
        // 构建 API 路径
        let api_path = "send_private_msg";

        let json_data = r#"
        {
            "user_id": 2977926714,
            "message": "Bot启动"
        }"#;
        // 调用 Bot 实例的 send_post_request 方法
        self.send_post_request(api_path, json_data).await
    }
    pub async fn send_group_msg_api(&self) -> Result<String, reqwest::Error> {
        // 构建 API 路径
        let api_path = "send_group_msg";
        let json_data = r#"
        {
            "user_id": 2977926714,
            "message": "Bot启动"
        }"#;
        self.send_post_request(api_path, json_data).await
    }
}
