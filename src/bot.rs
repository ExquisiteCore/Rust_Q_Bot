use std::time::Duration;

use tokio_tungstenite::WebSocketStream;
use tokio::{net::TcpStream,time::sleep};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use url::Url;



pub async fn connect(url: &str) -> WebSocketStream<TcpStream> {
    let server_url = Url::parse(url).expect("Failed to parse WebSocket URL");
    let tcp_stream = TcpStream::connect(server_url.host_str().unwrap().to_string() + ":5800")
        .await
        .expect("Failed to connect to server");

    tokio_tungstenite::client_async(server_url, tcp_stream)
        .await
        .expect("Failed to establish WebSocket connection")
        .0
}
pub struct Bot {
    ws_server_url: String,
    http_server_url: String,
    client: reqwest::Client,
    pub ws_stream: WebSocketStream<TcpStream>,
    reconnecting: bool,
}
impl Bot {
    pub async fn new(server_url: &str,wsport:i32,httpport:i32) -> Bot {

        let ws_server_url = format!("ws://{}:{}", server_url, wsport);

        let http_server_url = format!("http://{}:{}/", server_url, httpport);

        let ws_stream = connect(&ws_server_url)
            .await;
        Bot {
            ws_server_url,
            http_server_url,
            ws_stream ,
            client: reqwest::Client::new(),
            reconnecting : false,
        }
    }
    //ws返回json数据
    pub async fn receive_message(&mut self) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        //self.ws_stream.next().await
        loop{
            match self.ws_stream.next().await {
                Some(Ok(message)) => {
                    // 成功接收消息时，重置 reconnecting 字段
                    self.reconnecting = false;
                    return Some(Ok(message));
                }
                Some(Err(err)) => {
                    eprintln!("Error receiving message: {:?}", err);

                    // 处理错误，例如可以尝试重新连接
                    println!("WebSocket连接已断开,尝试重新连接");
                    self.start_reconnect().await;
                }
                None => {
                    // 当连接断开时，尝试重新连接
                    println!("WebSocket连接已断开,尝试重新连接");
                    self.start_reconnect().await;
                }
            }
            }
        }

    //post请求
    pub async fn send_post_request(&self, api: &str,json_data: &str) ->Result<String, reqwest::Error> {
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
    
    // 关闭 WebSocket 连接
    pub async fn close_websocket(&mut self) {
        // 使用 WebSocketStream 的 close 方法，无需特定的 CloseFrame
        if let Err(err) = self.ws_stream.close(None).await {
            eprintln!("关闭 WebSocket 连接时发生错误: {:?}", err);
        }
    }

    pub async fn start_reconnect(&mut self) {
        // 防止同时进行多次重连
        if !self.reconnecting {
            self.reconnecting = true;

            println!("尝试重新连接...");

            // 重新连接前等待一段时间
            sleep(Duration::from_secs(5)).await;

            // 尝试重新连接
            self.ws_stream = connect(&self.ws_server_url).await;

            // 重置 reconnecting 字段
            self.reconnecting = false;
        }
    }
}