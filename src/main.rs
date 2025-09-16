use anyhow::bail;
use clap::Parser;
use git2::{Cred, CredentialType, PushOptions, RemoteCallbacks, Repository, Signature};

#[derive(Parser)]
struct Cli {
    tag: String,
    #[arg(short, long)]
    remote: bool,
}

use git2::ObjectType;

pub fn delete_remote_tag(
    repo: &Repository,
    remote_name: &str,
    tag_name: &str,
) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote(remote_name)?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, allowed| {
        if allowed.contains(CredentialType::SSH_KEY) {
            return Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"));
        }
        if allowed.contains(CredentialType::DEFAULT) {
            return Cred::default();
        }
        Err(git2::Error::from_str("No suitable credentials available"))
    });
    callbacks.push_update_reference(|refname, status| {
        if let Some(msg) = status {
            eprintln!("Server rejected update for {}: {}", refname, msg);
        }
        Ok(())
    });

    let mut opts = PushOptions::new();
    opts.remote_callbacks(callbacks);

    let delete_spec = format!(":refs/tags/{}", tag_name);
    remote.push(&[delete_spec.as_str()], Some(&mut opts))?;
    Ok(())
}

fn re_tag(repo: &Repository, tag: &str, new_target_spec: &str, remote: bool) -> anyhow::Result<()> {
    let refname = format!("refs/tags/{tag}");

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

    let _ = repo.tag_delete(tag);
    if remote {
        delete_remote_tag(repo, "origin", tag)?;
    }

    let target = repo.revparse_single(new_target_spec)?;
    repo.tag(tag, &target, &tagger, &message, false)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Cli { tag, remote } = Cli::parse();
    let repo = Repository::discover(".")?;
    re_tag(&repo, &tag, "HEAD", remote)?;
    Ok(())
}
