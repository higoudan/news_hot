use std::{str::FromStr, time::Duration};
use chrono::Utc;
use tokio::time;
use cron::Schedule;
use serde::Deserialize;

#[tokio::main]
async  fn main() {
   //               sec  min   hour   day of month   month   day of week   year
  let expression = "0   */1   *   *   *  *  *";
  let schedule = Schedule::from_str(expression).unwrap();
  println!("Upcoming fire times:");
  for datetime in schedule.upcoming(Utc).take(10) {
    println!("-> {}", datetime);
  }

        // 获取当前时间作为起点
        // let mut now = time::Instant::now();
        let mut date = Utc::now();

    loop {
        // 计算下一个触发点
        let next = schedule.upcoming(Utc).next().expect("msg");

        // 获取距离下次触发的时间
        let duration_until_next = next.signed_duration_since(&date).to_std().unwrap();

        // 异步等待直到下一次触发
        time::sleep(duration_until_next).await;

        // 执行定时任务
        execute_task().await;

        // 更新当前时间为上次任务完成的时间
        date = Utc::now();
    }
}


async fn execute_task() {
    println!("Task executed at: {}", chrono::Local::now());
    let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build().unwrap();
    let response = client.get("https://www.toutiao.com/api/pc/feed/?category=news_hot")
    .send()
    .await.unwrap()
    .text()
    .await.unwrap();

    let data = match serde_json::from_str::<Data>(&response) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return ();
        }
    };
    // 组装发送到钉钉群聊
    let hots = data.data;

    if hots.len() <= 0 {
        return ;
    }
    let ch_hots: Vec<Vec<NewHot>> = hots.chunks(10).map(|chunk| chunk.to_vec()).collect();
    for ele in ch_hots {
        // 美10条组装为一个消息发送
        let markdownBody = sample_markdown_combo(ele);
        // 发送钉钉消息
        // client.post("url").header("1", 2).body(markdownBody);
    }


     // println!("response : {}", response);

}


async fn sample_markdown_combo(hots: Vec<NewHot>) -> String {

    for hot in hots {

    }
    return String::new();
}


#[derive(Debug, Deserialize, Clone)]
struct NewHot {
    media_avatar_url: String,
    title: String,
    #[serde(rename = "abstract")]
    abstract_desc: String,
    source_url: String,
    source: String,
}

#[derive(Debug, Deserialize)]
struct MaxBehotTime {
    max_behot_time: i32,
}

#[derive(Debug, Deserialize)]
struct Data {
    has_more: bool,
    message: String,
    data: Vec<NewHot>,
    next: MaxBehotTime,
}