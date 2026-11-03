use std::fmt;

use anyhow::Ok;
use anyhow::Result;
use clap::{Parser, ValueEnum};

const BASE_URL: &str = "https://xkcd.com";
const LATEST_COMIC: usize = 0;

/// A utility to grab XKCD comics
#[derive(Parser, Debug)]
pub struct Args {
    /// Set a connection timeout
    #[clap(long, short, default_value = "30")]
    pub timeout: u64,
    /// Print output in a format
    #[clap(long, short, value_enum, default_value = "text")]
    pub output: OutFormat,
    /// The comic to load
    #[clap(long, short)]
    pub num: Option<usize>,
    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum OutFormat {
    Json,
    Text,
}

pub struct XkcdClient {
    args: Args,
}
impl XkcdClient {
    pub fn new(args: Args) -> Self {
        XkcdClient { args }
    }
    pub fn run(&self) -> Result<()> {
        let url = if let Some(n) = self.args.num {
            format!("{}/{}/info.0.json", BASE_URL, n)
        } else {
            format!("{}/info.0.json", BASE_URL)
        };

        let http_client = reqwest::blocking::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(self.args.timeout))
            .build()?;
        let resp: ComicResponse = http_client.get(&url).send()?.text()?.try_into()?;
        let comic: Comic = resp.into();

        if self.args.save {
            comic.save()?;
        }
        comic.print(self.args.output)?;
        Ok(())
    }
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct ComicResponse {
    month: String,
    num: usize,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

impl TryFrom<String> for ComicResponse {
    type Error = anyhow::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json).map_err(|e| e.into())
    }
}

#[derive(Debug, serde_derive::Serialize)]
struct Comic {
    title: String,
    num: usize,
    date: String,
    desc: String,
    img_url: String,
}

impl From<ComicResponse> for Comic {
    fn from(cr: ComicResponse) -> Self {
        Comic {
            title: cr.title,
            num: cr.num,
            date: format!("{}-{}-{}", cr.year, cr.month, cr.day),
            desc: cr.alt,
            img_url: cr.img,
        }
    }
}
impl fmt::Display for Comic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Comic #{}: {} ({})\n{}\nImage URL: {}",
            self.num, self.title, self.date, self.desc, self.img_url
        )
    }
}
impl Comic {
    fn print(&self, out_format: OutFormat) -> Result<()> {
        match out_format {
            OutFormat::Text => println!("{}", self),
            OutFormat::Json => println!("{}", serde_json::to_string_pretty(self)?),
        }
        Ok(())
    }
    fn save(&self) -> Result<()> {
        Ok(())
    }
}
