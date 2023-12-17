use serde_json::Value;
pub fn process_meta_event(json: &Value) {
    // 处理 meta_event 类型的逻辑
    // 获取 meta_event_type 值
    if let Some(meta_event_type) = json["meta_event_type"].as_str() {
        match meta_event_type {
            "lifecycle" => println!("生活周期"),
            "heartbeat" => println!("心跳一次"),
            _ => println!("Unknown meta_event_type: {:?}", meta_event_type),
        }
    } else {
        println!("Missing meta_event_type field in meta_event");
    }
}