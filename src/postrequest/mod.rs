use serde_json::{Map, Value};

// 请求结构体
pub struct Request {
    http_server_url: String, // HTTP服务器URL
    client: reqwest::Client, // HTTP客户端
}

// 请求类方法
impl Request {
    // 发送POST请求
    pub async fn send_post_request(
        &self,
        api: &str,       // API路径
        json_data: &str, // JSON格式的数据
    ) -> Result<String, reqwest::Error> {
        // 发送 HTTP POST 请求
        match self
            .client
            .post(format!("{}{}", self.http_server_url, api))
            .header("Content-Type", "application/json")
            .header("authorization", "123")
            .body(json_data.to_owned())
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    // 成功获取响应
                    match response.text().await {
                        Ok(text) => Ok(text),
                        Err(err) => {
                            eprintln!("Error reading response text: {:?}", err);
                            Err(err)
                        }
                    }
                } else {
                    eprintln!(
                        "HTTP request failed with status code: {}",
                        response.status()
                    );
                    Err(response.error_for_status().unwrap_err())
                }
            }
            Err(err) => {
                eprintln!("Error sending HTTP request: {:?}", err);
                Err(err)
            }
        }
    }

    // 创建新的Request实例
    pub async fn new(server_url: &str, httpport: i32) -> Self {
        let http_server_url = format!("http://{}:{}/", server_url, httpport);
        Request {
            http_server_url,
            client: reqwest::Client::new(),
        }
    }
}

// Request类高级封装
impl Request {
    // 发送私聊消息API请求
    pub async fn send_private_msg_api(
        &self,
        user_id: &i64, // 用户ID
        message: &str, // 消息内容
    ) -> Result<String, reqwest::Error> {
        // 构建 API 路径
        let api_path = "send_private_msg";
        let json = synthesis_json(&[
            ("user_id", Value::Number(serde_json::Number::from(*user_id))),
            ("message", Value::String(message.to_string())),
        ]);
        self.send_post_request(api_path, &json.to_string().as_str())
            .await
    }

    // 发送群消息API请求
    pub async fn send_group_msg_api(
        &self,
        group_id: &i64, // 群ID
        message: &str,  // 消息内容
    ) -> Result<String, reqwest::Error> {
        // 构建 API 路径
        let api_path = "send_group_msg";
        let json = synthesis_json(&[
            (
                "group_id",
                Value::Number(serde_json::Number::from(*group_id)),
            ),
            ("message", Value::String(message.to_string())),
            ("auto_escape", Value::Bool(true)),
        ]);
        self.send_post_request(api_path, &json.to_string().as_str())
            .await
    }
}

// 合成JSON数据
fn synthesis_json(args: &[(&str, Value)]) -> Value {
    let mut json_map: Map<String, Value> = Map::new();

    for (key, value) in args {
        json_map.insert(key.to_string(), value.clone());
    }

    Value::Object(json_map)
}
