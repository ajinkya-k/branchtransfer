use git2::{ErrorClass, ErrorCode, Repository, Worktree, WorktreeAddOptions};
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

pub(crate) fn clean_worktree(wt: Worktree) -> anyhow::Result<()> {
    let _ = remove_dir_all(&wt.path())?;
    let _ = match wt.is_prunable(None)? {
        true => wt.prune(None),
        false => Err(git2::Error::new(
            ErrorCode::GenericError,
            ErrorClass::Worktree,
            &format!(
                "Couldn't prune worktree {} formerly opened at {}",
                wt.name().unwrap_or(""),
                wt.path().display()
            ),
        )),
    }?;
    Ok(())
}
