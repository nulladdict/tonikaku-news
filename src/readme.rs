use std::{fs, io::Write};

use anyhow::Result;

use crate::post::Post;

pub fn update_readme(posts: &[Post]) -> Result<()> {
    let posts = posts
        .iter()
        .map(|post| {
            let date = post.real_date.format("%d.%m.%Y");
            let text = format!("- [{}]({}) ({date})", post.title, post.link);
            return text;
        })
        .collect::<Vec<_>>()
        .join("\n");
    let content = format!("# ad-hoc интересности о фронтенде, вебе и не только\n\n{posts}\n");

    let mut readme = fs::File::create("./README.md")?;
    readme.write_all(content.as_bytes())?;

    Ok(())
}
