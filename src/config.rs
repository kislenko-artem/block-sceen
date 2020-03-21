use std::fs::File;
use std::time::Duration;
use std::io;

use dirs;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Config {
    pub big_cicle: Duration,
    pub small_cicle: Duration,
    pub big_break: Duration,
    pub smalll_break: Duration,
    // pub tips: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        let path = dirs::home_dir().unwrap();
        let path = path.join("block-screen.json");
        if path.exists() {
            let mut contents = String::new();
            let mut f = File::open(path.clone()).unwrap();
            f.read_to_string(&mut contents).expect("Something went wrong reading the file");
            let conf = serde_json::from_str(&contents).unwrap();
            return conf;
        }

        let mut conf = Config {
            big_cicle: Duration::from_secs(60 * 60),
            small_cicle: Duration::from_secs(60 * 15),
            big_break: Duration::from_secs(60 * 5),
            smalll_break: Duration::from_secs(15),
            // tips: Vec::new(),
        };

        // conf.tips.push(String::from("Первый совет"));
        // conf.tips.push(String::from("Второй совет"));
        // conf.tips.push(String::from("Третий совет"));

        let data = serde_json::to_string(&conf).unwrap();
        let mut f = File::create(path.clone()).unwrap();
        f.write_all(data.as_bytes()).unwrap();

        return conf;


    }
}