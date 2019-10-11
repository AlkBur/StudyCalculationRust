pub fn task(){
    println!("Num task: {}", 10);
    println!("{}", "-----------");

    print(1.7);
    print(3.0);
}

fn print(x:f64){
    println!("(x+1)^2+3(x+1)={}", f64::powf(x+1.0, 2.0)+3.0*(x+1.0));
}