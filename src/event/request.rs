use serde_json::Value;

pub async fn process_request_event(json: &Value, request: &crate::postrequest::Request) {
    if let Some(request_type) = json["request_type"].as_str() {
        match request_type {
            "friend" => process_friend_request(json, request),
            "group" => process_group_request(json),
            _ => println!("未知 request_type: {:?}", request_type),
        }
    } else {
        println!("Missing request_type field in request event");
    }
}

fn process_friend_request(json: &Value, request: &crate::postrequest::Request) {
    if let (Some(user_id), Some(comment)) = (json["user_id"].as_i64(), json["comment"].as_str()) {
        println!(
            "处理好友请求：\n\
            用户 QQ: {}\n\
            添加理由: {}",
            user_id, comment
        );
        // 在这里添加处理好友请求的具体逻辑
        //request.send_post_request(api, json_data)
    } else {
        println!("好友请求字段缺失或类型不匹配");
    }
}

fn process_group_request(json: &Value) {
    if let (Some(group_id), Some(user_id), Some(comment)) = (
        json["group_id"].as_i64(),
        json["user_id"].as_i64(),
        json["comment"].as_str(),
    ) {
        println!(
            "处理群请求：\n\
            群号: {}\n\
            申请者 QQ: {}\n\
            附加信息: {}",
            group_id, user_id, comment
        );
        // 在这里添加处理群请求的具体逻辑
    } else {
        println!("群请求字段缺失或类型不匹配");
    }
}
