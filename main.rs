fn main() {
    println!("Hello, World!");
    println!("Welcome to Rust world!");
}

pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}")
}
