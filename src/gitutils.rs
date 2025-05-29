use git2::{ErrorCode, Repository, Worktree, WorktreeAddOptions};
use std::{fs::remove_dir_all, path::Path};

pub(crate) fn create_worktree(
    repo: &Repository,
    branch: &String,
    path: &Path,
) -> anyhow::Result<Worktree> {
    let _ = repo.is_bare().then(|| {}).ok_or(git2::Error::new(
        git2::ErrorCode::Invalid,
        git2::ErrorClass::Repository,
        "Repository is bare!",
    ));

    let mut opts = WorktreeAddOptions::new();
    opts.checkout_existing(true);
    let wt = repo.worktree(branch, &path, Some(&opts))?;

    Ok(wt)
}

pub(crate) fn show_branch(repo: &Repository) -> anyhow::Result<()> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e.into()),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    println!(
        "On branch {}",
        head.unwrap_or("Not currently on any branch")
    );
    Ok(())
}

pub(crate) fn clean_worktree(wt: Worktree) -> anyhow::Result<()> {
    let _ = remove_dir_all(&wt.path())?;
    let _ = wt.is_prunable(None).and_then(|_| wt.prune(None))?;
    Ok(())
}
