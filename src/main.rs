use anyhow::Result;
use branchtransfer::branch_transfer;
mod parser;
use parser::parse_args;

fn main() -> Result<()> {
    let (path, srcrel, branch, trgrel, msg) = parse_args()?;
    branch_transfer(path, branch, srcrel, trgrel, msg)
}
