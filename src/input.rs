use colored::*;
use std::io;

fn ask_user() -> String {
    let mut buf = String::new();
    let error_msg = "Invalid input.".red();

    println!("Enter the topic: ");

    io::stdin().read_line(&mut buf).expect(&error_msg);

    buf
}

fn topic_check() {
    match ask_user().trim().parse::<u8>() {
        Ok(1) => println!("{}", "topic 1".green()),
        Ok(2) => println!("{}", "topic 2".green()),
        Ok(3) => println!("{}", "topic 3".green()),
        Ok(4) => println!("{}", "topic 4".green()),
        Ok(5) => println!("{}", "topic 5".green()),
        _ => println!("{}", "Not a valid number".red()),
    }
}

pub fn impl_input() {
    topic_check();
}
