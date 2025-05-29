use anyhow::Result;
use std::{env::current_dir, io::ErrorKind, ops::Not, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// branch to copy to
    #[arg(long, short)]
    branch: Option<String>,
    /// directory to copy
    srcdir: String,
    /// commit message
    #[arg(long, short)]
    message: Option<String>,

    #[arg(long, short)]
    trgdir: Option<String>,
}

pub(crate) fn parse_args() -> Result<(PathBuf, String, String, String, String)> {
    let args = Cli::parse();
    let srcrel = args
        .srcdir
        .contains("..")
        .not()
        .then(|| args.srcdir)
        .ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                "Absoulute path must not contain '..'",
            )
        })?;
    let trgrel = match args.trgdir {
        Some(x) => x.contains("..").not().then(|| x).ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                "target rel path must not contain '..'",
            )
        })?,
        None => srcrel.clone(),
    };
    let branch = args.branch.unwrap_or_else(|| "gh-pages".to_string());

    let path = current_dir()?;
    let msg = args
        .message
        .unwrap_or_else(|| format!("Replacing the contents of {} with {}", trgrel, srcrel));
    Ok((path, srcrel, branch, trgrel, msg))
}
