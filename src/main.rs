use clap::Parser;

#[derive(Parser)]
struct Cli {
    tag: String,
}

fn main() -> anyhow::Result<()> {
    let Cli { tag: tag_string } = Cli::parse();
    let repo = git2::Repository::open(".")?;
    let tag = repo.revparse_single(&format!("tags/{tag_string}"))?;
    repo.tag_delete(&tag_string)?;

    repo.tag(
        &tag_string,
        &repo.revparse_single("HEAD")?,
        &repo.signature()?,
        tag.as_tag()
            .expect("Not a valid tag!")
            .message()
            .unwrap_or_default(),
        false,
    )?;

    println!("Hello, world!");
    Ok(())
}
