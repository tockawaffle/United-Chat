use crate::chat::youtube::structs::youtube_response::YoutubeResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VideoInfo {
    pub(crate) is_replay: Option<bool>,
    pub(crate) api_key: Option<String>,
    pub(crate) stream_type: Option<String>,
    pub(crate) continuation: Option<String>,
    pub(crate) scheduled_start_time: Option<String>,
    pub(crate) client_version: Option<String>,
    pub(crate) video_id: Option<String>,
}

fn retrieve_video_info(html: &str) -> Result<VideoInfo, String> {
    let mut video_info = VideoInfo {
        is_replay: None,
        api_key: None,
        stream_type: None,
        continuation: None,
        scheduled_start_time: None,
        client_version: None,
        video_id: None,
    };

    let re = regex::Regex::new(r#""isReplay"\s*:\s*(true)"#).unwrap();
    if let Some(replay) = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())) {
        if replay == "true" {
            return Err("The video is a replay and cannot be set as a live video".to_string());
        }
        video_info.is_replay = Some(true);
    }

    let re = regex::Regex::new(r#""INNERTUBE_API_KEY"\s*:\s*"([^"]+)""#).unwrap();
    video_info.api_key = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    if video_info.api_key.is_none() {
        return Err("Cannot find the API key".to_string());
    }

    let re = regex::Regex::new(r#""continuation"\s*:\s*"([^"]+)""#).unwrap();
    video_info.continuation = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    if let Some(continuation) = &video_info.continuation {
        video_info.stream_type = Some("live".to_string());
    } else {
        let re = regex::Regex::new(r#""scheduledStartTime"\s*:\s*"([^"]+)""#).unwrap();
        video_info.scheduled_start_time = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

        if video_info.scheduled_start_time.is_some() {
            video_info.stream_type = Some("scheduled".to_string());
        } else {
            return Err("Cannot find the continuation".to_string());
        }
    }

    let re = regex::Regex::new(r#""clientVersion"\s*:\s*"([\d.]+)""#).unwrap();
    video_info.client_version = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    if video_info.client_version.is_none() {
        return Err("Cannot find the client version".to_string());
    }

    let re = regex::Regex::new(r#"<link\s+rel="canonical"\s+href="https://www\.youtube\.com/watch\?v=([^"]+)""#).unwrap();
    video_info.video_id = re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    if video_info.video_id.is_none() {
        return Err("Cannot find the video ID".to_string());
    }

    Ok(video_info)
}

async fn get_video(id: &str) -> VideoInfo {
    let request_url = format!("https://www.youtube.com/watch?v={}", id);
    let request = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0")
        .build()
        .unwrap()
        .get(&request_url)
        .send()
        .await
        .unwrap();

    // Get the raw data
    let response = request.text().await.unwrap();
    let raw_data = retrieve_video_info(&response).unwrap();
    raw_data
}

fn preprocess_data(data: Vec<Value>) -> Vec<Value> {
    data.into_iter()
        .filter(|x| x.get("clientId").is_some())
        .collect()
}

fn parse_message_type(data: &Vec<Value>) -> Result<Vec<YoutubeResponse>, ()> {
    let mut responses = Vec::new();

    // Access the "addChatItemAction" key
    let chat_items: Vec<Value> = data.iter()
        .filter_map(|x| x.get("addChatItemAction").cloned())
        .collect();

    // Serialize and deserialize the data
    let json_string = serde_json::to_string(&chat_items).map_err(|_| ())?;
    let processed_data: Vec<Value> = serde_json::from_str(&json_string).map_err(|_| ())?;

    for item in processed_data {
        if let Some(message) = item.get("item")
            .and_then(|i| i.get("liveChatTextMessageRenderer"))
            .and_then(|m| m.as_object()) {
            let author_name = message.get("authorName")
                .and_then(|a| a.get("simpleText"))
                .and_then(|a| a.as_str())
                .unwrap_or("Unknown Author")
                .to_string();


            // Get badges if any
            let badges_urls: Vec<String> = message.get("authorBadges")
                .and_then(|b| b.as_array())
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|x| x.get("liveChatAuthorBadgeRenderer"))
                .filter_map(|b| b.get("customThumbnail"))
                .filter_map(|c| c.get("thumbnails"))
                .filter_map(|t| t.get(0))
                .filter_map(|t| t.get("url"))
                .filter_map(|u| u.as_str())
                .map(|u| u.to_string())
                .collect();

            // Messages and emojis
            let message_text = message.get("message")
                .and_then(|m| m.get("runs"))
                .and_then(|r| r.as_array())
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|item| {
                    if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                        Some(text.to_string())
                    } else if let Some(emoji) = item.get("emoji").and_then(|e| e.as_object()) {
                        let emoji_url = emoji.get("image")
                            .and_then(|i| i.get("thumbnails"))
                            .and_then(|t| t.get(0))
                            .and_then(|t| t.get("url"))
                            .and_then(|u| u.as_str())
                            .unwrap_or("Unknown Emoji URL");

                        let emoji_name = emoji.get("image")
                            .and_then(|i| i.get("accessibility"))
                            .and_then(|a| a.get("accessibilityData"))
                            .and_then(|d| d.get("label"))
                            .and_then(|l| l.as_str())
                            .unwrap_or("Unknown Emoji");

                        Some(format!("<img id=\"{}\" src=\"{}\" alt=\"{}\" />", emoji_name, emoji_url, emoji_name))
                    } else {
                        None
                    }
                }).collect::<Vec<String>>().join(" ");

            let timestamp = message.get("timestampUsec")
                .and_then(|t| t.as_str())
                .unwrap_or("Unknown Timestamp")
                .to_string();

            let tracking_params = message.get("trackingParams")
                .and_then(|t| t.as_str())
                .unwrap_or("Unknown Tracking Params")
                .to_string();

            let author_id = message.get("authorExternalChannelId")
                .and_then(|t| t.as_str())
                .unwrap_or("Unknown Author ID")
                .to_string();

            let response = YoutubeResponse {
                author_id,
                author_name,
                author_badges: badges_urls,
                message: message_text,
                message_emotes: Vec::new(), // Modify this as needed
                timestamp,
                tracking_params,
            };

            responses.push(response);
        }
    }

    Ok(responses)
}


async fn get_live_chat(data: VideoInfo) -> Result<(Vec<YoutubeResponse>, String), String> {
    let continuation = data.continuation.unwrap();
    let api_key = data.api_key.unwrap();
    let client_version = data.client_version.unwrap();

    let url = format!("https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={}", api_key);

    let request = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0")
        .build()
        .unwrap()
        .post(&url)
        .json(&json!({
                "context": {
                    "client": {
                        "clientName": "WEB",
                        "clientVersion": client_version,
                    }
                },
                "continuation": continuation,
            }))
        .send()
        .await
        .unwrap();


    let text_response = request.text().await.unwrap();
    let json_response: Value = serde_json::from_str(&text_response).unwrap();
    // println!("{:?}", response);
    let action = json_response["continuationContents"]["liveChatContinuation"]["actions"].as_array().unwrap();

    if action.is_empty() {
        return Err("Cannot find the chat".to_string());
    }

    let continuation_data = json_response["continuationContents"]["liveChatContinuation"]["continuations"].as_array().unwrap();

    if continuation_data.is_empty() {
        return Err("Cannot find the continuation".to_string());
    }

    // Can be either invalidationContinuationData or timedContinuationData
    let mut continuation_type = String::new();
    match continuation_data[0]["timedContinuationData"].as_object() {
        Some(_) => {
            let timed_continuation = continuation_data[0]["timedContinuationData"]["continuation"].as_str().unwrap();
            continuation_type = timed_continuation.to_string();
        }
        None => {
            let invalidation_continuation = continuation_data[0]["invalidationContinuationData"]["continuation"].as_str().unwrap();
            continuation_type = invalidation_continuation.to_string();
        }
    }

    let message = parse_message_type(&action).unwrap();

    Ok((message, continuation_type))
}

async fn youtube_polling(interval: u64, live_id: &str) {
    let video = get_video(live_id).await;
    let mut continuation = get_live_chat(video.clone()).await.unwrap().1;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        let data = get_live_chat(video.clone()).await.unwrap().0;
        continuation = get_live_chat(video.clone()).await.unwrap().1;
        println!("{:?}", data);
    }
}


#[tauri::command]
pub(crate) async fn get_video_cmd(id: String) -> VideoInfo {
    get_video(&id).await
}

#[tauri::command]
pub(crate) async fn get_live_chat_cmd(video: VideoInfo) -> Result<(Vec<YoutubeResponse>, String), String> {
    get_live_chat(video).await
}

#[tauri::command]
pub(crate) async fn youtube_polling_cmd(interval: u64, live_id: String) {
    youtube_polling(interval, &live_id).await;
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_video_cmd() {
        let video = get_video("8OlZQTSq63I").await;
        println!("{:?}", video);
    }

    #[tokio::test]
    async fn test_get_live_chat_cmd() {
        let video = get_video("WrW-QlNG1eo").await;
        get_live_chat(video).await.expect("TODO: panic message");
    }

    #[tokio::test]
    async fn test_youtube_polling() {
        youtube_polling(5, "WrW-QlNG1eo").await;
    }
}

