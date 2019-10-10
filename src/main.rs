extern crate chrono;
use chrono::prelude::*;

use std::env;

fn main() {
    let mut num_task = 0;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        num_task = args[1].parse().unwrap();
        //println!("The first argument is {}", args[1]);
    }

    match num_task {
        1 => task_1(),
        2 => task_2(),
        _ => {
            task_1();
            task_2()
        }
    }


    //println!("Num task {}", num_task);
}

fn task_1(){
    println!("Num task: {}", 1);
    println!("Silence is golden");
}

/*
Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Weekday::Mon => write!(f, "Monday"),
            Weekday::Tue => write!(f, "Tuesday"),
            Weekday::Wed => write!(f, "Wednesday"),
            Weekday::Thu => write!(f, "Thursday"),
            Weekday::Fri => write!(f, "Friday"),
            Weekday::Sat => write!(f, "Saturday"),
            Weekday::Sun=> write!(f, "Sunday"),
        }
    }
}
*/

fn task_2(){
    println!("Num task {}", 2);


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
}
