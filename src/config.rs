use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::IpAddr;
use std::path::Path;
use std::{collections::BTreeMap, ops::Deref};
use warp::http::Uri;

#[derive(Debug, Deserialize)]
pub struct ValidUri(#[serde(deserialize_with = "uri_from_str")] Uri);

impl Serialize for ValidUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.0.to_string())
    }
}

impl Deref for ValidUri {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: IpAddr,
    pub port: u16,
    pub threads: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    pub title: String,
    pub text_color: String,
    pub bg_color_top: String,
    pub bg_color_bottom: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub index: Index,
    pub urls: BTreeMap<String, ValidUri>,
}

fn uri_from_str<'de, D>(deserializer: D) -> Result<Uri, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(D::Error::custom)
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();

        let f = File::open(path)?;
        let mut br = BufReader::new(f);
        let mut buf: Vec<u8> = Vec::new();

        br.read_to_end(&mut buf)?;
        let config: Self = toml::from_slice(&buf)?;

        Ok(config)
    }
}
