use std::io;
use chrono::prelude::*;
use::colored::Colorize;

fn main() {
    display_current_time();

    set_alarm_sequence();
}

fn display_current_time() {
    println!("{} {}", "Current Time:".green(), Local::now().format("%H:%M:%S %d-%m-%Y "));
}


fn set_alarm_sequence() {
    println!("What time do you want to set an alarm for? (format HH:MM)");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let input = input.trim();

            match NaiveTime::parse_from_str(input, "%H:%M") {
                Ok(time) => {
                    println!("{:?}", time);
                }
                Err(e) => println!("Failed to parse time: {}", e),
            }
        },
        Err(e) => println!("Error{}", e),
    }
}