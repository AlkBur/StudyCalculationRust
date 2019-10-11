pub fn task(){
    println!("Num task: {}", 11);
    println!("{}", "-----------");

    print(-2.34);
}

fn print(x:f64){
    let mut val1 = f64::abs(x-5.0);
    val1 -= f64::sin(x);
    val1 /=3.0;

    let mut val2 = x.powf(2.0);
    val2 += 2014.0;
    val2 = val2.sqrt()*f64::cos(2.0*x);


    println!("(|x−5|−sin x)/3+sqrt(x^2+2014)*cos 2x - 3={}", val1+val2-3.0);
}