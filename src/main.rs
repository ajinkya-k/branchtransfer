use anyhow::Result;
use branchtransfer::{branch_transfer, branch_xf_old};

fn main() -> Result<()> {
    println!("Hello, world!");
    branch_transfer();
    branch_xf_old()
}
