use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

#[derive(Clone, Debug)]
pub struct Post {
    pub title: String,
    pub link: String,
    pub real_date: NaiveDate,
    pub pub_time: DateTime<Utc>,
}

impl Post {
    pub fn from(path: &PathBuf) -> Result<Self> {
        let file_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .context(format!("invalid file name {path:?}"))?;

        let title = get_title(&path)?;
        let link =
            format!("https://github.com/nulladdict/tonikaku-news/blob/main/posts/{file_name}.md");
        let real_date = NaiveDate::parse_from_str(file_name, "%Y-%m-%d")
            .context(format!("invalid date {file_name}"))?;
        let pub_time = get_last_modified_time(&path)?;

        Ok(Post {
            title,
            link,
            real_date,
            pub_time,
        })
    }
}

lazy_static! {
    static ref TITLE_REGEX: Regex = RegexBuilder::new(r"^# (.*)$")
        .multi_line(true)
        .build()
        .unwrap();
}

fn get_title(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(&path).context("cannot read file {path:?}")?;
    let title = TITLE_REGEX
        .captures(&content)
        .and_then(|captures| captures.get(1))
        .context(format!("title not found for {path:?}"))?
        .as_str()
        .to_string();
    Ok(title)
}

fn get_last_modified_time(path: &PathBuf) -> Result<DateTime<Utc>> {
    let metadata = fs::metadata(path)?;
    let last_modified = metadata.modified()?;
    let last_modified = DateTime::<Utc>::from(last_modified);
    Ok(last_modified)
}
