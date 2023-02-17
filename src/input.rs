use std::io;

pub fn ask_user() -> String {
    let mut buf = String::new();

    println!("Enter the topic (default: all): ");

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    input
}
