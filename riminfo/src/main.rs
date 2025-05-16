use std::path::Path;

use anyhow::Result;
use anyhow::bail;

struct RimInfo {
    pub revision: String,
}

fn main() {
    println!("Hello, from RimInfo!");
}

fn from_dir(path: &Path) -> Result<RimInfo> {
    bail!("Error while reading .riminfo from path");
}
