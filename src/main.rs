use dictionary;

fn main() {
    println!("Hello, world!");
    println!("Lorem Ipsum.....");
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    print_rim_details();
    println!(
        "If url starts with {} then we are talking to Gerrit server",
        dictionary::GERRIT_SERVER
    );
    println!("{} is important in rim calculation", dictionary::CHECKSUM);
}

fn print_rim_details() {
    println!("Riminfo file name {}", dictionary::RIM_INFO);
}
