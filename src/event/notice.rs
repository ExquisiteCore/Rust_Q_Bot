use serde_json::Value;

use crate::postrequest;
pub fn process_notice_event(json: &Value) {
    // 处理 notice 类型的逻辑
    // 提取 notice_type 值
    if let Some(notice_type) = json["notice_type"].as_str() {
        match notice_type {
            "friend_recall" => friend_recall(json),
            "group_recall" => group_recall(json),
            "group_increase" => group_increase(json),
            "group_decrease" => group_decrease(json),
            "group_admin" => group_admin(json),
            "group_upload" => group_upload(json),
            "private_upload" => private_upload(json),
            "group_ban" => group_ban(json),
            "group_card" => group_card(json),
            "essence" => essence(json),
            "notify" => {
                if let Some(sub_type) = json["sub_type"].as_str() {
                    match sub_type {
                        "poke" => process_poke(json),
                        "title" => process_title(json),
                        _ => println!("Unknown notify sub_type: {:?}", sub_type),
                    }
                } else {
                    println!("Missing sub_type field in notify event");
                }
            }
            _ => println!("Unknown notice_type: {:?}", notice_type),
        }
    } else {
        println!("Missing notice_type field in notice event");
    }
}
fn friend_recall(json: &Value) {
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let operator_id = json["operator_id"].as_i64().unwrap_or_default();
    let message_id = json["message_id"].as_i64().unwrap_or_default();
    println!(
        "收到私聊消息撤回事件：用户 {} 撤回了消息 {}，操作者是 {}",
        user_id, message_id, operator_id
    );
}

fn group_recall(json: &Value) {
    let group_id = json["group_id"].as_i64().unwrap_or_default();
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let operator_id = json["operator_id"].as_i64().unwrap_or_default();
    let message_id = json["message_id"].as_i64().unwrap_or_default();
    println!(
        "收到群聊消息撤回事件：群 {} 中用户 {} 撤回了消息 {}，操作者是 {}",
        group_id, user_id, message_id, operator_id
    );
}
fn group_increase(json: &Value) {
    let group_id = json["group_id"].as_i64().unwrap_or_default();
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let operator_id = json["operator_id"].as_i64().unwrap_or_default();
    let sub_type = json["sub_type"].as_str().unwrap_or("未知类型");
    println!(
        "群组成员增加事件：群 {} 中新增成员 {}，操作者是 {}，子类型是 {}",
        group_id, user_id, operator_id, sub_type
    );
}

fn group_decrease(json: &Value) {
    let group_id = json["group_id"].as_i64().unwrap_or_default();
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let operator_id = json["operator_id"].as_i64().unwrap_or_default();
    let sub_type = json["sub_type"].as_str().unwrap_or("未知类型");
    println!(
        "群组成员减少事件：群 {} 中减少成员 {}，操作者是 {}，子类型是 {}",
        group_id, user_id, operator_id, sub_type
    );
}

fn group_admin(json: &Value) {
    let group_id = json["group_id"].as_i64().unwrap_or_default();
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let sub_type = json["sub_type"].as_str().unwrap_or("未知类型");
    println!(
        "群组管理员变动事件：群 {} 中成员 {} 的管理员权限发生变动，子类型是 {}",
        group_id, user_id, sub_type
    );
}

fn group_upload(json: &Value) {
    let group_id = json["group_id"].as_i64().unwrap_or_default();
    let user_id = json["user_id"].as_i64().unwrap_or_default();
    let file_id = json["file"]["id"].as_str().unwrap_or("未知文件ID");
    let file_name = json["file"]["name"].as_str().unwrap_or("未知文件名");
    let file_size = json["file"]["size"].as_i64().unwrap_or_default();
    let file_url = json["file"]["url"].as_str().unwrap_or("未知下载地址");
    println!(
        "群文件上传事件：群 {} 中用户 {} 上传了文件:ID={}, 名称={}, 大小={}, 下载地址={}",
        group_id, user_id, file_id, file_name, file_size, file_url
    );
}


fn private_upload(json: &Value) {
    if let (Some(user_id), Some(sender), Some(private_file)) =
        (json["user_id"].as_i64(), json["sender"].as_i64(), json["private_file"].as_object())
    {
        if let (Some(file_id), Some(file_name), Some(file_size), Some(file_url), Some(sub_id), Some(expire)) = (
            private_file["id"].as_str(),
            private_file["name"].as_str(),
            private_file["size"].as_i64(),
            private_file["url"].as_str(),
            private_file["sub_id"].as_str(),
            private_file["expire"].as_i64(),
        ) {
            println!(
                "私聊文件上传事件：\n\
                上传者 QQ: {}\n\
                发送者 QQ: {}\n\
                文件信息:\n\
                - 文件ID: {}\n\
                - 文件名: {}\n\
                - 文件大小: {} 字节\n\
                - 下载地址: {}\n\
                - 子文件ID: {}\n\
                - 文件过期时间: {}",
                user_id, sender, file_id, file_name, file_size, file_url, sub_id, expire
            );
            return;
        }
    }
    print_error(json, "字段缺失或类型不匹配");
}

fn group_ban(json: &Value) {
    if let (Some(group_id), Some(user_id), Some(operator_id), Some(duration), Some(sub_type)) = (
        json["group_id"].as_i64(),
        json["user_id"].as_i64(),
        json["operator_id"].as_i64(),
        json["duration"].as_i64(),
        json["sub_type"].as_str(),
    ) {
        println!(
            "群禁言事件：\n\
            群号: {}\n\
            被禁言成员 QQ: {}\n\
            操作者 QQ: {}\n\
            禁言时长: {} 秒\n\
            子类型: {}",
            group_id, user_id, operator_id, duration, sub_type
        );
        return;
    }
    print_error(json, "字段缺失或类型不匹配");
}

fn group_card(json: &Value) {
    if let (Some(group_id), Some(user_id), Some(card_new), Some(card_old)) = (
        json["group_id"].as_i64(),
        json["user_id"].as_i64(),
        json["card_new"].as_str(),
        json["card_old"].as_object(),
    ) {
        if let (Some(old_name), Some(old_size), Some(old_url)) = (
            card_old["name"].as_str(),
            card_old["size"].as_i64(),
            card_old["url"].as_str(),
        ) {
            println!(
                "群成员名片变动事件：\n\
                群号: {}\n\
                变动成员 QQ: {}\n\
                新名片: {}\n\
                旧名片文件信息:\n\
                - 文件名: {}\n\
                - 文件大小: {} 字节\n\
                - 下载链接: {}",
                group_id, user_id, card_new, old_name, old_size, old_url
            );
            return;
        }
    }
    print_error(json, "字段缺失或类型不匹配");
}

fn essence(json: &Value) {
    if let (Some(group_id), Some(sender_id), Some(operator_id), Some(message_id), Some(sub_type)) = (
        json["group_id"].as_i64(),
        json["sender_id"].as_i64(),
        json["operator_id"].as_i64(),
        json["message_id"].as_i64(),
        json["sub_type"].as_str(),
    ) {
        println!(
            "精华消息事件：\n\
            群号: {}\n\
            发送者 QQ: {}\n\
            操作者 QQ: {}\n\
            消息 ID: {}\n\
            子类型: {}",
            group_id, sender_id, operator_id, message_id, sub_type
        );
        return;
    }
    print_error(json, "字段缺失或类型不匹配");
}

fn process_poke(json: &Value) {
    if let (Some(user_id), Some(sender_id), Some(group_id), Some(target_id), Some(poke_detail)) = (
        json["user_id"].as_i64(),
        json["sender_id"].as_i64(),
        json["group_id"].as_i64(),
        json["target_id"].as_i64(),
        json["poke_detail"].as_object(),
    ) {
        if let (Some(action), Some(suffix), Some(action_img_url)) = (
            poke_detail["action"].as_str(),
            poke_detail["suffix"].as_str(),
            poke_detail["action_img_url"].as_str(),
        ) {
            println!(
                "头像戳一戳事件：\n\
                发送者 QQ: {}\n\
                好友 QQ (仅私聊): {}\n\
                群号 (仅群聊): {}\n\
                被戳者 QQ: {}\n\
                戳一戳的详细信息:\n\
                - 操作名称: {}\n\
                - 后缀: {}\n\
                - 操作图标: {}",
                user_id, sender_id, group_id, target_id, action, suffix, action_img_url
            );
            return;
        }
    }
    print_error(json, "字段缺失或类型不匹配");
}

fn process_title(json: &Value) {
    if let (Some(group_id), Some(user_id), Some(title)) = (
        json["group_id"].as_i64(),
        json["user_id"].as_i64(),
        json["title"].as_str(),
    ) {
        println!(
            "群头衔变更事件：\n\
            群号: {}\n\
            操作者 QQ: {}\n\
            新头衔: {}",
            group_id, user_id, title
        );
        return;
    }
    print_error(json, "字段缺失或类型不匹配");
}
fn print_error(json: &Value, message: &str) {
    println!("事件处理失败，{}:\n{}", message, json);
}