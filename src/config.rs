pub struct Config {
    pub consumer_key: String,
    pub access_token: String,
}

pub fn get_config() -> Config {
    let consumer_key = std::env::var("POCKET_CONSUMER_KEY").unwrap();
    let access_token = std::env::var("POCKET_ACCESS_TOKEN").unwrap();
    Config {
        consumer_key,
        access_token,
    }
}
