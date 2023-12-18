mod bot;
mod event;
mod postrequest;

use postrequest::Request;
use bot::Bot;
use tokio::signal::ctrl_c;
use tokio_tungstenite::tungstenite::Message;
use event::process_json;
#[tokio::main]
async fn main() {
    
    let server_url = "192.168.1.8";
    let request = Request::new(server_url, 5700).await;
    let mut bot = Bot::new(server_url, 5800,).await;
    


    match request.send_private_msg_api(&2977926714u32,"hello").await{
        Ok(text) => {
            println!("post 返回{}", text);
        }
        Err(err) => {
            eprintln!("Error sending HTTP request: {:?}", err);
        }
    }

    let mut exit_flag = false;

    // tokio::select! 允许我们等待多个异步任务
    tokio::select! {
        _ = ctrl_c() => {
            // 收到 Ctrl+C 信号
            exit_flag = true;
        }
        _ = async {
            while !exit_flag {
                // 处理传入的消息
                match bot.receive_message().await {
                    Some(Ok(message)) => {
                        if let Message::Text(text) = message {
                            let json = text;
                            process_json(&json);
                        }
                    }
                    Some(Err(err)) => {
                        eprintln!("Error receiving message: {:?}", err);
                        // 处理错误，可能需要重试或退出循环
                    }
                    _ => {}
                }
            }
        } => {},
        
    };

    if exit_flag {
        bot.close_websocket().await;
        println!("Exiting...");
    }
}
