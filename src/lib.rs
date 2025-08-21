use std::{
    fs,
    io::{Error, ErrorKind},
    path::{PathBuf, absolute},
};

use git2::{ErrorClass, ErrorCode, Repository, StatusOptions};

mod fileops;
mod gitutils;
use fileops::{copy_all, rm_contents};
use gitutils::{clean_worktree, create_worktree};
use tempfile::tempdir;
pub fn branch_transfer(
    repopath: PathBuf,
    branch: String,
    srcrel: String,
    trgrel: String,
    msg: String,
) -> anyhow::Result<()> {
    // open repo
    // let repopath = Path::new(&path);
    let repo = Repository::open(&repopath)?; //TODO: switch with discover
    let repopath = repo.path().parent().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            "Problem computing the parent of the repo root",
        )
    })?;

    let src = repopath.join(srcrel); //TODO: Change accordingly with Repo::discover
    println!("{:?}", repopath);
    // let branch = "rootfs".to_string();
    let tdir = tempdir()?;
    let rpath = tdir.path().join("wt-".to_string() + &branch);

    println!("Attemtping to create worktree at {}", rpath.display());
    let wt = create_worktree(&repo, &branch, &rpath)?;
    println!("Created wt: {:?}", wt.path());

    // switch to worktree
    let repo = Repository::open(rpath)?;

    // print branch name
    match repo.head() {
        Ok(head) if head.is_branch() => println!(
            "On branch {}",
            head.shorthand()
                .unwrap_or_else(|| "couldn't resolve ref to shortname")
        ),
        Ok(_) => {
            clean_worktree(wt)?;
            return Err(git2::Error::new(
                ErrorCode::Invalid,
                ErrorClass::Reference,
                format!("HEAD resolved to non-branch ref. Cleaning up and exiting!"),
            )
            .into());
        }
        Err(e) if e.code() == ErrorCode::UnbornBranch => {
            println!("On Unborn Branch {}", &branch)
        }
        Err(e) => {
            clean_worktree(wt)?;
            return Err(e.into());
        }
    };
    let targrel = &trgrel;
    let trg = absolute(wt.path().join(targrel))?;
    println!("{:?}", trg);
    println!("Target directory: {:?}", trg);

    if (targrel != ".") & fs::exists(&trg)? {
        println!("Cleaning and recreating directory");
        fs::remove_dir_all(&trg)?;
        fs::create_dir_all(&trg)?;
    } else if targrel == "." {
        println!("removing contents of worktree");
        rm_contents(&trg)?;
    }
    println!("Attempting Copy!");
    match copy_all(&src, &trg) {
        Ok(_) => println!("Successfully copied"),
        Err(e) => {
            clean_worktree(wt)?;
            return Err(e)
        }
    };

    // Get status
    let mut stopt = StatusOptions::new();
    println!("{:?}", targrel.to_string() + "/*");
    stopt.pathspec(&targrel.to_string());

    let st = repo.statuses(None)?;

    if st.is_empty() {
        // if no change, cleanup, and prune worktree
        println!("Nothing to do, Cleaning up worktree ");
        clean_worktree(wt)?;
        return Ok(());
    }

    // there are some changed files, add them
    let mut index = repo.index()?;
    // for fl in st.iter().filter(|e| e.status() != git2::Status::CURRENT) {
    //     let ptadd = PathBuf::from(fl.path().unwrap());
    //     // index.add_path(&ptadd)?;
    //     // index.add_all(PathBuf::from(targrel).iter(), git2::IndexAddOption::DEFAULT, None).unwrap();
    // }
    // write index
    index.add_all(
        PathBuf::from(targrel).iter(),
        git2::IndexAddOption::DEFAULT,
        None,
    )?;
    index.write()?;

    // create tree
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = repo.signature()?;

    // prev commit = current HEAD
    let pcmt = repo.head()?.peel_to_commit()?;

    // actually do the commiting
    println!("Commiting");
    repo.commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&pcmt])?;

    // cleanup and prune
    clean_worktree(wt)?;

    Ok(())
}
