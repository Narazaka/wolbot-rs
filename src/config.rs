use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub channel_id: String,
    pub mac: String,
}

impl Channel {
    pub fn channel_id(&self) -> u64 {
        self.channel_id.parse::<u64>().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub channels: Vec<Channel>,
}

impl Config {
    pub fn get_channel(&self, channel_id: u64) -> Option<&Channel> {
        self.channels.iter().find(|c| c.channel_id() == channel_id)
    }
}

pub fn read(file_name: &str) -> Result<Config> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Config = serde_json::from_reader(reader).unwrap();
    Ok(t)
}
