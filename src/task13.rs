pub fn task(){
    println!("Num task: {}", 13);
    println!("{}", "-----------");

    print(3.6);
}

fn print(x:f64){
    let e = 1.0_f64.exp();
    let mut val = e.powf(x-2.0);
    val += f64::abs(f64::sin(x));
    val -= f64::powf(x, 4.0)*f64::cos(1.0/x);

    println!("e^(x−2)+|sin(x)|−x^4⋅cos(1/x)={}", val);
}