pub fn task() {
    println!("Num task {}", 3);
    println!("{}", "-----------");

    for number in 1..6 {
        let str = "0".repeat(number);
        println!("{}. {}", number, str);
    }
}