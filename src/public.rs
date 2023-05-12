use std::{fs, io::Write};

use anyhow::Result;
use rss::{Channel, Item};

use crate::post::Post;

pub fn generate_public(posts: &[Post]) -> Result<()> {
    fs::create_dir_all("./public")?;
    add_index(posts)?;
    add_feed(posts)?;
    Ok(())
}

fn add_index(posts: &[Post]) -> Result<()> {
    let latest_posts = posts
        .iter()
        .rev()
        .take(8)
        .map(|post| {
            format!(
                r#"<li><a href="{}">{}</a> (<time datetime="{}">{}</time>)</li>"#,
                post.link,
                post.title,
                post.real_date.format("%Y-%m-%d"),
                post.real_date.format("%d.%m.%Y")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let last_post = posts.len();
    let mut index = fs::File::create("./public/index.html")?;
    let content = format!(
        r#"
<!DOCTYPE html>
<html lang="ru">
<head>
  <meta charset="utf-8">
  <title>Tonikaku News</title>
  <link rel="alternate" type="application/rss+xml" title="RSS" href="./feed.rss" />
  <style>
    :root {{
      font-family: -apple-system,BlinkMacSystemFont,"Segoe UI","Noto Sans",Helvetica,Arial,sans-serif;
    }}
  </style>
</head>
<body>
  <article>
    <h1>ad-hoc интересности о фронтенде, вебе и не только</h1>
    <main>
      <p>Весь контент <a href="https://github.com/nulladdict/tonikaku-news/blob/main/README.md">живет на гитхабе</a>, вот последние посты:</p>
      <ol reversed start="{last_post}">
        {latest_posts}
      </ol>
      <p>Этот сайт нужен только чтобы <a href="./feed.rss">подписаться по RSS</a></p>
    </main>
  </article>
</body>
</html>"#
    );
    index.write_all(content.as_bytes())?;
    Ok(())
}

fn add_feed(posts: &[Post]) -> Result<()> {
    let channel = Channel {
        title: "Tonikaku News".into(),
        link: "https://github.com/nulladdict/tonikaku-news".into(),
        description: "ad-hoc интересности о фронтенде, вебе и не только".into(),
        language: Some("ru-ru".into()),
        items: posts
            .iter()
            .map(|post| Item {
                title: Some(post.title.clone()),
                link: Some(post.link.clone()),
                pub_date: Some(post.pub_time.to_rfc2822()),
                ..Item::default()
            })
            .collect(),
        ..Default::default()
    };

    let mut rss_feed = fs::File::create("./public/feed.rss")?;
    channel.pretty_write_to(&mut rss_feed, b' ', 2)?;
    rss_feed.write(b"\n")?;

    Ok(())
}
