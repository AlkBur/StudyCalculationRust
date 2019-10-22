mod task1; mod task2; mod task3; mod task4;
mod task5; mod task6; mod task7; mod task8;
mod task9; mod task10; mod task11; mod task12;
mod task13;

use std::env;

//export RUSTFLAGS='-Ctarget-cpu=native'

fn main() {
    let mut num_task = 0;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        num_task = args[1].parse().unwrap();
    }

    match num_task {
        1 => task1::task(), 2 => task2::task(),
        3 => task3::task(), 4 => task4::task(),
        5 => task5::task(), 6 => task6::task(),
        7 => task7::task(), 8 => task8::task(),
        9 => task8::task(), 10 => task10::task(),
        11 => task10::task(), 12 => task12::task(),
        13 => task13::task(),
        _ => {
            task1::task();
            println!("{}", "-----------"); task2::task();
            println!("{}", "-----------"); task3::task();
            println!("{}", "-----------"); task4::task();
            println!("{}", "-----------"); task5::task();
            println!("{}", "-----------"); task6::task();
            println!("{}", "-----------"); task7::task();
            println!("{}", "-----------"); task8::task();
            println!("{}", "-----------"); task9::task();
            println!("{}", "-----------"); task10::task();
            println!("{}", "-----------"); task11::task();
            println!("{}", "-----------"); task12::task();

        }
    }
}
