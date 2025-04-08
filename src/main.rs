use anyhow::Context;
use anyhow::Result;

fn main() {
    println!("Hello, world!");
    println!("For testing the new changes in service");
}

fn fetch_repo() -> Result<git2::Repository> {
    git2::Repository::open(".").context("Current working directory is not a git repo")
}

fn some_random_function() {
    println!("Hello");
}
