use anyhow::Result;
use branchtransfer::branch_transfer;
mod parser;
use parser::parse_args;

fn main() -> Result<()> {
    let (path, srcrel, branch, trgrel, msg) = parse_args()?;
    branch_transfer(path, branch, srcrel, trgrel, msg)
    // branch_transfer(
    //     ".".to_string(),
    //     "rootfs".to_string(),
    //     "f1".to_string(),
    //     "s4".to_string(),
    //     format!("Copied contents from a different worktree into s3"),
    // )
}
