use serde_json::Value;

pub async fn process_message_event(json: &Value, request: &crate::postrequest::Request) {
    // 处理 message 类型的逻辑
    // 提取消息类型和子类型
    let message_type = json["message_type"].as_str().unwrap_or("未知消息类型");
    let sub_type = json["sub_type"].as_str().unwrap_or("未知消息子类型");

    // 根据消息类型和子类型进行分类处理
    match (message_type, sub_type) {
        ("private", "friend") => process_private_friend_message(json, request).await,
        ("group", "normal") => process_group_normal_message(json),
        _ => println!(
            "Unknown message_type or sub_type: {} - {}",
            message_type, sub_type
        ),
    }
}

pub async fn process_message_send_event(json: &Value, request: &crate::postrequest::Request) {
    // 处理 message_sent 类型的逻辑
    // 提取消息内容
    let message_content = json["message"][0]["data"]["text"]
        .as_str()
        .unwrap_or_default();

    // 提取发送者昵称
    let sender_nickname = json["sender"]["nickname"].as_str().unwrap_or("未知用户");

    // 打印消息信息
    println!("你发送了消息给 {}:\n{}", sender_nickname, message_content);
}
async fn process_private_friend_message(json: &Value, request: &crate::postrequest::Request) {
    // 提取私聊消息的发送者昵称和消息内容
    let sender_nickname = json["sender"]["nickname"].as_str().unwrap_or("未知用户");
    let sender_user_id = json["sender"]["user_id"].as_i64().unwrap_or(1);
    let message_content = json["message"][0]["data"]["text"]
        .as_str()
        .unwrap_or("未知消息内容");
    // 打印私聊消息信息
    println!("{}发送了消息:{}", sender_nickname, &message_content);
    //复读
    if let "测试" = message_content {
        if let Ok(text) = request
            .send_private_msg_api(&sender_user_id, &message_content)
            .await
        {}
    }
}
fn process_group_normal_message(json: &Value) {
    // 提取群消息的发送者群名片、昵称和消息内容
    let sender_nickname = json["sender"]["nickname"].as_str().unwrap_or("未知用户");
    let message_content = json["message"][0]["data"]["text"]
        .as_str()
        .unwrap_or("未知消息内容");

    // 打印群消息信息
    println!(
        "来自{}的{}发送了消息:{}",
        json["group_id"], sender_nickname, message_content
    );
}
