use anyhow::Result;
use branchtransfer::branch_transfer;

fn main() -> Result<()> {
    println!("Hello, world!");
    branch_transfer(
        ".".to_string(),
        "rootfs".to_string(),
        "f1".to_string(),
        "s4".to_string(),
        format!("Copied contents from a different worktree into s3"),
    )
}
