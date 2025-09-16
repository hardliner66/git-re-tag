use anyhow::bail;
use clap::Parser;
use git2::{Repository, Signature};

#[derive(Parser)]
struct Cli {
    tag: String,
}

use git2::ObjectType;

fn retag(repo: &Repository, tag_name: &str, new_target_spec: &str) -> anyhow::Result<()> {
    let refname = format!("refs/tags/{tag_name}");

    // Find the existing tag ref
    let obj = repo.revparse_single(&refname)?;

    // If it's an annotated tag object, copy its tagger + message.
    // Otherwise (lightweight), synthesize metadata.
    let (tagger, message) = if obj.kind() == Some(ObjectType::Tag) {
        let tag = repo.find_tag(obj.id())?.clone();
        let sig = tag
            .tagger()
            .or_else(|| repo.signature().ok())
            .unwrap_or(Signature::now("unknown", "unknown@example.com")?)
            .to_owned();
        let msg = tag.message().unwrap_or("").to_string();
        (sig, msg)
    } else {
        bail!("Tag not found!")
    };

    let _ = repo.tag_delete(tag_name);

    let target = repo.revparse_single(new_target_spec)?;
    repo.tag(tag_name, &target, &tagger, &message, false)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Cli { tag } = Cli::parse();
    let repo = Repository::discover(".")?;
    retag(&repo, &tag, "HEAD")?;
    Ok(())
}
