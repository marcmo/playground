use dictionary;

fn main() {
    println!("Hello, world!");
    println!("Lorem Ipsum.....");
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    print_rim_details();
}

fn print_rim_details() {
    println!("Riminfo file name {}", dictionary::RIM_INFO);
}
