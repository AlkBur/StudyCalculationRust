extern crate chrono;
use chrono::prelude::*;

pub fn task(){
    println!("Num task {}", 2);
    println!("{}", "-----------");

    let now = Local::now();
    match now.weekday().number_from_monday() {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        _ => println!("Sunday")
    }
    match now.month() {
        1 => println!("January"),
        2 => println!("February"),
        3 => println!("March"),
        4 => println!("April"),
        5 => println!("May"),
        6 => println!("June"),
        7 => println!("July"),
        8 => println!("August"),
        9 => println!("September"),
        10 => println!("October"),
        11 => println!("November"),
        _ => println!("December")
    }
    println!("Alexander")
}