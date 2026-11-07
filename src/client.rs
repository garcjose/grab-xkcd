use anyhow::Ok;
use anyhow::Result;
use std::fmt;
use std::io::Write;
use url::Url;

use crate::cli::XkcdArgs;
use crate::cli::OutFormat;

const BASE_URL: &str = "https://xkcd.com";

pub struct XkcdClient {
    args: XkcdArgs,
}
impl XkcdClient {
    pub fn new(args: XkcdArgs) -> Self {
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
        // reqwest re-exports the url crate's Url type
        //let url = reqwest::Url::parse(&self.img_url)?;
        let url = Url::parse(&self.img_url)?;
        let image_name = url.path_segments().unwrap().last().unwrap();

        let current_dir = std::env::current_dir()?;
        //let file_name = format!("xkcd-{}.png", self.num);
        let file_path = current_dir.join(image_name);
        let mut file = std::fs::File::create(file_path)?;

        let resp = reqwest::blocking::get(&self.img_url)?;
        // copy using std::io::copy to write (requires mut resp and mut file)
        //std::io::copy(&mut resp, &mut file)?;
        // write using write_all (requires mut file)
        file.write_all(&resp.bytes()?)?;

        Ok(())
    }
}
