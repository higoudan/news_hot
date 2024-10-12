use chrono::Utc;
use config::{Config, File};
use cron::Schedule;
use std::{str::FromStr, time::Duration};
use tokio::time;

mod config_model;

mod model;

#[tokio::main]
async fn main() {
    // 通过配置获取定时cron
    let mut c = Config::builder()
        .add_source(File::with_name("Config"))
        .build()
        .unwrap();

    let config: config_model::Config = c.try_deserialize().unwrap();

    //               sec  min   hour   day of month   month   day of week   year
    //   let expression = "0   */1   *   *   *  *  *";
    let schedule = Schedule::from_str(&config.init.cron).unwrap();
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
        execute_task(&config).await;

        // 更新当前时间为上次任务完成的时间
        date = Utc::now();
    }
}

async fn execute_task(config: &config_model::Config) {
    println!("Task executed at: {}", chrono::Local::now());
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let response = client
        .get(&config.newshot.base_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let data = match serde_json::from_str::<model::Data>(&response) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return ();
        }
    };
    // 组装发送到钉钉群聊
    let hots = data.data;

    if hots.is_empty() {
        return;
    }
    let ch_hots: Vec<Vec<model::NewHot>> = hots.chunks(20).map(|chunk| chunk.to_vec()).collect();

    if ch_hots.is_empty() {
        return;
    }
    let frist = &ch_hots[0];

    // 获取组装的 markdown 的数据
    let mark_Str = sample_markdown_combo(&config.newshot.perfix_url,frist).await;


    let message  = model::MarkdownMessage::new(String::from("头条热搜"), mark_Str);

    let sendData = model::DingtalkSendData::new(message);

    let response = client
        .post(&config.newshot.dingtalk_url)
        .json(&sendData)  
        .send()
        .await.unwrap();
    print!("请求钉钉放回{:?}", response);

}

/// 组装markdown文档
async fn sample_markdown_combo(perfixUrl: &str, hots: &Vec<model::NewHot>) -> String {
    let mut context = String::from("###  头条热搜 \n");
    for  hot in hots {
        context.push_str("--- \n ");
        // context.push_str(string);
        context.push_str("- #### ");
        let url = click_url(perfixUrl, &hot);
        context.push_str( &url);
        context.push_str("\n");
        
        // context.push_str("- ##### ");
        // context.push_str( &hot.abstract_desc);
        // context.push_str("\n");
    }
    return context;
}

/// 链接组装
fn click_url(perfixUrl: &str, hot: &model::NewHot) -> String {
    let result = "[{title}]({url})";
    let result = result.replace("{title}", &hot.title);
    let url = perfixUrl.to_owned() + &hot.source_url;
    result.replace("{url}", &url)
}
