use std::io;

pub fn ask_user() -> String {
    let mut input = String::new();

    println!("Enter the topic (default: all): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}
