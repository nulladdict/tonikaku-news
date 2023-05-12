use anyhow::Result;
use glob::glob;
use post::Post;
use public::generate_public;
use readme::update_readme;

mod post;
mod public;
mod readme;

fn main() -> Result<()> {
    let mut posts = glob("./posts/*.md")?
        .map(|path| -> Result<Post> { Post::from(&path?) })
        .collect::<Result<Vec<_>>>()?;
    posts.sort_by(|a, b| a.real_date.cmp(&b.real_date));

    update_readme(&posts)?;
    generate_public(&posts)?;

    Ok(())
}
