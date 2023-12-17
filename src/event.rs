mod message;
mod notice;
mod request;
mod meta_event;

use serde_json::Value;

pub fn process_json(json_str: &str) {
    // 使用 serde_json 解析 JSON 字符串
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");

    // 获取 post_type 值
    let post_type = json["post_type"].as_str();
    // 根据 post_type 值进行分类处理
    match post_type {
        Some("message") => message::process_message_event(&json),
        Some("message_sent") => message::process_message_sent_event(&json),
        Some("notice") => notice::process_notice_event(&json),
        Some("request") => request::process_request_event(&json),
        Some("meta_event") => meta_event::process_meta_event(&json),
        _ => println!("未知 post_type: {:?}", post_type),
    }
}