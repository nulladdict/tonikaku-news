use std::{fs, io::Write};

use anyhow::Result;
use leptos::*;
use rss::{Channel, Guid, Item};

use crate::post::Post;

pub fn generate_public(posts: &[Post]) -> Result<()> {
    fs::create_dir_all("./public")?;
    add_index(posts)?;
    add_feed(posts)?;
    Ok(())
}

fn add_index(posts: &[Post]) -> Result<()> {
    leptos_dom::HydrationCtx::stop_hydrating();
    let posts = posts.to_vec();
    let content = ssr::render_to_string(move || view! { <Document posts /> });
    let mut index = fs::File::create("./public/index.html")?;
    index.write_all(b"<!DOCTYPE html>")?;
    index.write_all(content.as_bytes())?;
    index.write_all(b"\n")?;
    Ok(())
}

#[component]
fn Document(posts: Vec<Post>) -> impl IntoView {
    view! {
        <html lang="ru">
            <Head />
            <Body posts />
        </html>
    }
}

#[component]
fn Head() -> impl IntoView {
    view! {
        <head>
            {r#"<meta charset="utf-8">"#}
            <title>Tonikaku News</title>
            {r#"<meta name="viewport" content="width=device-width, initial-scale=1">"#}
            {r#"<meta name="description" content="ad-hoc интересности о фронтенде, вебе и не только">"#}
            <link rel="alternate" type="application/rss+xml" title="RSS" href="./feed.rss" />
            <style>{r#":root{{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","Noto Sans",Helvetica,Arial,sans-serif;}}"#}</style>
        </head>
    }
}

#[component]
fn Body(posts: Vec<Post>) -> impl IntoView {
    let last_post = posts.len();
    let content_href = "https://github.com/nulladdict/tonikaku-news/blob/main/README.md";
    let rss_href = "./feed.rss";
    view! {
        <body>
            <article>
                <h1>"ad-hoc интересности о фронтенде, вебе и не только"</h1>
                <main>
                    <p>"Весь контент "<a href=content_href>"живёт на гитхабе"</a>", вот последние посты:"</p>
                    <ol reversed start=last_post><LatestPosts posts /></ol>
                    <p>"Этот сайт нужен только чтобы "<a href=rss_href>"подписаться по RSS"</a></p>
                </main>
            </article>
        </body>
    }
}
#[component]
fn LatestPosts(posts: Vec<Post>) -> impl IntoView {
    posts
        .iter()
        .rev()
        .take(8)
        .cloned()
        .map(|post| view! { <Post post /> })
        .collect_view()
}

#[component]
fn Post(post: Post) -> impl IntoView {
    let date_attr = post.real_date.format("%Y-%m-%d").to_string();
    let date_text = post.real_date.format("%d.%m.%Y").to_string();
    view! {
        <li>
            <a href=post.link>{post.title}</a>" ("<time datetime=date_attr>{date_text}</time>")"
        </li>
    }
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
                guid: Some(Guid {
                    value: post.link.clone(),
                    permalink: true,
                }),
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
