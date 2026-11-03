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
    #[clap(long, short, default_value = "0")]
    pub num: usize,
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

#[derive(Debug)]
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
            date: format!("{}-{}-{}", cr.day, cr.year, cr.month),
            desc: cr.alt,
            img_url: cr.img,
        }
    }
}
