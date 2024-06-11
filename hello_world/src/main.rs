fn main() {
    println!("Hello, world!");
    print_str("Satoshi Nakamoto");

    let str = String::from("HogeHoge");
    print_str(str.as_str());
}

fn print_str(str: &str) {
    println!("Hello, world {}!", str);
}