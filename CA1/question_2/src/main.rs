//student loan repayment estimator

use std::io;

fn main() {
    loop {
    let mut p = String::new();
    let mut r = String::new();
    let mut t = String::new();

    let mut input = String::new();
    println!("Calculate for new borrower? (y or n): ");
    io::stdin().read_line(&mut input).expect("Not a valid input");
    input = input.trim().to_lowercase();
    if input == "n" {
        break
    } else if input == "y" {
        println!("Welcome");
    } else {
        println!("Invalid response");
    }

    println!("Input borrowed amount: ");
    io::stdin().read_line(&mut p).expect("Not a valid input");
    let p:f32 = p.trim().parse().expect("Not a valid number");

    println!("Input rate (in%): ");
    io::stdin().read_line(&mut r).expect("Not a valid input");
    let r:f32 = r.trim().parse().expect("Not a valid number");

    println!("Input time in years: ");
    io::stdin().read_line(&mut t).expect("Not a valid input");
    let t:f32 = t.trim().parse().expect("Not a valid number");

    let a:f32 = p * (1.0 + r/100.0).powf(t);
    let ci:f32 = a - p;
    println!("Amount due is: {}", ci);

}
}