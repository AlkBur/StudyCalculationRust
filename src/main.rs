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
        3 => task_3(),
        4 => task_4(),
        5 => task_5(),
        _ => {
            task_1();
            println!("{}", "-----------");
            task_2();
            println!("{}", "-----------");
            task_3();
            println!("{}", "-----------");
            task_4();
            println!("{}", "-----------");
            task_5();
            println!("{}", "-----------");
        }
    }


    //println!("Num task {}", num_task);
}

fn task_1(){
    println!("Num task: {}", 1);
    println!("{}", "-----------");
    println!("Silence is golden");
}

fn task_2(){
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

fn task_3() {
    println!("Num task {}", 3);
    println!("{}", "-----------");

    for number in 1..6 {
        let str = "0".repeat(number);
        println!("{}. {}", number, str);
    }
}

fn task_4() {
    println!("Num task {}", 4);
    println!("{}", "-----------");

    for _number in 1..6 {
        let str = "A".repeat(8);
        println!("{}", str);
    }
}

fn task_5() {
    println!("Num task {}", 5);
    println!("{}", "-----------");

    println!("{}", "*     *     *");
    println!("{}", " *   * *   * ");
    println!("{}", "  * *   * *  ");
    println!("{}", "   *     *   ");
}