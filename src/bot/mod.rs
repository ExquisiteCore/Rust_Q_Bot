use std::time::Duration;

use futures_util::StreamExt;
use tokio::{net::TcpStream, time::sleep};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use url::Url;
// 连接 WebSocket
pub async fn connect(url: &str) -> WebSocketStream<TcpStream> {
    // 解析 WebSocket URL
    let server_url = Url::parse(url).expect("Failed to parse WebSocket URL");

    // 获取主机地址和端口
    let host = server_url
        .host_str()
        .expect("No host found in the given URL");
    let port = server_url
        .port_or_known_default()
        .expect("No port found in the given URL");

    // 构建主机地址字符串，包含端口
    let host_with_port = format!("{}:{}", host, port);

    // 使用 TcpStream::connect 时直接传入解析得到的地址
    let tcp_stream = TcpStream::connect(host_with_port)
        .await
        .expect("Failed to connect to server");

    // 使用解析得到的地址和 TcpStream 建立 WebSocket 连接
    tokio_tungstenite::client_async(server_url, tcp_stream)
        .await
        .expect("Failed to establish WebSocket connection")
        .0
}
pub struct Bot {
    ws_server_url: String,
    ws_stream: WebSocketStream<TcpStream>,
    reconnecting: bool,
}

impl Bot {
    // 创建 Bot 实例
    pub async fn new(server_url: &str, wsport: i32) -> Self {
        let ws_server_url = format!("ws://{}:{}", server_url, wsport);

        let ws_stream = connect(&ws_server_url).await;
        Bot {
            ws_server_url,
            ws_stream,
            reconnecting: false,
        }
    }
    // 接收 WebSocket 的消息
    pub async fn receive_message(
        &mut self,
    ) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        loop {
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

    // 关闭 WebSocket 连接
    pub async fn close_websocket(&mut self) {
        // 使用 WebSocketStream 的 close 方法，无需特定的 CloseFrame
        if let Err(err) = self.ws_stream.close(None).await {
            eprintln!("关闭 WebSocket 连接时发生错误: {:?}", err);
        }
    }

    // 尝试重新连接
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
