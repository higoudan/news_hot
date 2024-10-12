use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub init: InitConfig,
    pub newshot: NewshotConfig,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    pub cron: String,
}

#[derive(Debug, Deserialize)]
pub struct NewshotConfig {
    pub base_url: String,
    pub perfix_url: String,
    pub dingtalk_url: String,
}


