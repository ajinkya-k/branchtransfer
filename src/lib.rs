use std::{
    fs,
    io::{Error, ErrorKind},
    path::{PathBuf, absolute},
};

use anyhow::Result;

use git2::{Repository, StatusOptions};

mod fileops;
mod gitutils;
use fileops::{copy_all, rm_contents};
use gitutils::{clean_worktree, create_worktree, show_branch};
use tempfile::tempdir;
pub fn branch_transfer(
    repopath: PathBuf,
    branch: String,
    srcrel: String,
    trgrel: String,
    msg: String,
) -> Result<()> {
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
    // panic!("Random path: {} ", rpath.display());
    //
    println!("Attemtping to create worktree at {}", rpath.display());
    let wt = create_worktree(&repo, &branch, &rpath)?;
    println!("Created wt: {:?}", wt.path());

    // switch to worktree
    let repo = Repository::open(rpath)?; //.unwrap_or_else(|_| panic!("could not open repo"));
    show_branch(&repo);
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
    let _ = copy_all(&src, &trg)?;

    // let paths = fs::read_dir(&trg).unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    // Get status
    let mut stopt = StatusOptions::new();
    println!("{:?}", targrel.to_string() + "/*");
    stopt.pathspec(&targrel.to_string()); //TODO: this may not work if targrel
    // let st = repo.statuses(Some(&mut stopt))?;
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
    index
        .add_all(
            PathBuf::from(targrel).iter(),
            git2::IndexAddOption::DEFAULT,
            None,
        )
        .unwrap();
    index.write()?;

    // create tree
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = repo.signature().unwrap();

    // prev commit = current HEAD
    let pcmt = repo.head().unwrap().peel_to_commit()?;

    // actually do the commiting
    println!("Commiting");
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg, // &format!("Copied contents from a different worktree into {}", targrel),
        &tree,
        &[&pcmt], // no parents
    )?;

    // cleanup and prune
    clean_worktree(wt)?;

    Ok(())
}
