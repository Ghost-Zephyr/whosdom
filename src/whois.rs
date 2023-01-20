use std::{
    io::{self, Read, Write},
    env::home_dir,
    vec::Vec,
    fs};

use whois_rust::{WhoIsLookupOptions, WhoIs};

use crate::open;

const SERVERS: &'static str = "https://raw.githubusercontent.com/FurqanSoftware/node-whois/master/servers.json";
const CACHE_FOLDER: &'static str = ".cache/whosdom";
const SERVER_FILE: &'static str = "servers.json";

#[derive(Debug)]
pub struct Watcher {
    pub domains: Vec<String>,
    whois: WhoIs,
}

impl Watcher {
    pub fn new(servers: String, domains: Vec<String>) -> Self {
        let whois = WhoIs::from_string(servers).unwrap();
        Self { domains, whois }
    }
    pub async fn watch(&mut self) -> ! {
        loop {
        }
    }
    pub async fn lookup(&self, domain: &str) -> String {
        self.whois.lookup_async(WhoIsLookupOptions::from_string(domain).unwrap()).await.unwrap()
    }
}

pub async fn get_servers() -> Result<String, reqwest::Error> {
    let path = match home_dir() {
        Some(home) => format!("{}/{CACHE_FOLDER}", home.display()),
        None => format!("/tmp/whosdom")
    };
    fs::create_dir_all(&path).unwrap();
    let mut file = open(&format!("{path}/{SERVER_FILE}")).unwrap();
    let mut servers = String::new();
    file.read_to_string(&mut servers);
    if servers.is_empty() {
        servers = reqwest::get(SERVERS).await?.text().await?;
        file.write_all(&servers.as_bytes()).unwrap();
    }
    Ok(servers)
}
