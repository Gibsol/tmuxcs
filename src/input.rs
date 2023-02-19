use colored::*;
use std::io;

use crate::topics::topics;

fn ask_user() -> String {
    let mut buf = String::new();
    let error_msg = "Invalid input.".red();

    println!("Enter the topic: ");

    io::stdin().read_line(&mut buf).expect(&error_msg);

    buf
}

fn topic_check() {
    match ask_user().trim().parse::<u8>() {
        Ok(1) => println!("{}", topics()[0].green()),
        Ok(2) => println!("{}", topics()[1].green()),
        Ok(3) => println!("{}", topics()[2].green()),
        Ok(4) => println!("{}", topics()[3].green()),
        Ok(5) => println!("{}", topics()[4].green()),
        _ => println!("{}", "Not a valid number".red()),
    }
}

pub fn impl_input() {
    topic_check();
}
