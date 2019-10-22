pub fn task(){
    println!("Num task: {}", 13);
    println!("{}", "-----------");

    print(0.1, 0.2, 1.0);
}

fn print(a:f64, b:f64, x:f64){
    let mut val = f64::powf(x.powf(2.0)+b, 1.0/5.0);
    val -= b.powf(2.0)*f64::sin(x+a).powf(3.0)/x;

    println!("sqrt(x^2+5)^5-b^2*sin(x+a)^3/x={}", val);
}