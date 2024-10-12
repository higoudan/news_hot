use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NewHot {
    pub media_avatar_url: String,
    pub title: String,
    #[serde(rename = "abstract")]
    pub abstract_desc: String,
    pub source_url: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct MaxBehotTime {
    pub max_behot_time: i32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub has_more: bool,
    pub message: String,
    pub data: Vec<NewHot>,
    pub next: MaxBehotTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DingtalkSendData {
    pub msgtype: String,
    pub markdown: MarkdownMessage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkdownMessage {
    pub title: String,
    pub text: String,
}


impl MarkdownMessage {
    
    pub fn new (title:String, text: String) -> Self {
        MarkdownMessage{
            title: title,
            text: text
        }
    }
}


impl DingtalkSendData {

    pub fn new( message: MarkdownMessage) -> Self {
        DingtalkSendData{
            msgtype: String::from("markdown"),
            markdown: message
        }
    }
}