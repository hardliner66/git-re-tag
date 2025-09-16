use clap::Parser;
use git2::{Repository, Signature};
use stringlit::s;

#[derive(Parser)]
struct Cli {
    #[arg(value_names = ["REMOTE?", "TAG"], num_args = 1..=2)]
    args: Vec<String>,
}

use git2::ObjectType;

fn re_tag(repo: &Repository, tag: &str, new_target_spec: &str) -> anyhow::Result<()> {
    let refname = format!("refs/tags/{tag}");

    let obj = repo.revparse_single(&refname)?;

    let meta = if obj.kind() == Some(ObjectType::Tag) {
        let tag_obj = repo.find_tag(obj.id())?.clone();
        let sig = tag_obj
            .tagger()
            .or_else(|| repo.signature().ok())
            .unwrap_or(Signature::now("unknown", "unknown@example.com")?)
            .to_owned();
        let msg = tag_obj.message().unwrap_or("").to_string();
        Some((sig, msg))
    } else {
        None
    };

    let _ = repo.tag_delete(tag);

    let target = repo.revparse_single(new_target_spec)?;

    if let Some((tagger, message)) = meta {
        repo.tag(tag, &target, &tagger, &message, false)?;
    } else {
        repo.tag_lightweight(tag, &target, false)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Cli { args } = Cli::parse();
    let (remote, tag) = if args.len() == 1 {
        (s!("origin"), args[0].to_owned())
    } else {
        (args[0].to_owned(), args[1].to_owned())
    };
    let repo = Repository::discover(".")?;
    re_tag(&repo, &tag, "HEAD")?;
    eprintln!("Tag successfully recreated");
    eprintln!("If the previous tag was already pushed, you need to run the following:");
    println!("git push --delete {remote} {tag}; git push {remote} {tag}");
    Ok(())
}
