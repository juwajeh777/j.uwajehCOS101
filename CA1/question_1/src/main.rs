// electricity bill estimator

use std::io;

fn main() {
    loop {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a valid input");

    let mut input = String::new();
    println!("Estimate new bill? (y or n): ");
    io::stdin().read_line(&mut input).expect("Not a valid input");
    input = input.trim().to_lowercase();
    if input == "n" {
        break
    } else if input == "y" {
        println!("Welcome");
    } else {
        println!("Invalid response");
    }

    println!("Input units consumed (kWh): ");
    let mut units = String::new();
    io::stdin().read_line(&mut units).expect("Not a valid input");
    let units:u32 = units.trim().parse().expect("Not a valid number");

    if units >= 500 {
        let rate = 50;
        let amount = (units * rate) + 5000;
        println!("Customer name: {}", name);
        print!("Units: {}", units);
        println!("Rate: {} (plus added 5000)", rate);
        println!("Total bill: {}", amount);
    } else if units >= 301 {
        let rate = 50;
        let amount = units * rate;
        println!("Customer name: {}", name);
        println!("Units: {}", units);
        println!("Rate: {}", rate);
        println!("Total bill: {}", amount);
    } else if units >= 101 {
        let rate = 35;
        let amount = units * 50;
        println!("Customer name: {}", name);
        println!("Units: {}", units);
        println!("Rate: {}", rate);
        println!("Total bill: {}", amount);
    } else if units >= 1 {
        let rate = 20;
        let amount = units * rate;
        println!("Customer name: {}", name);
        println!("Units: {}", units);
        println!("Rate: {}", rate);
        println!("Total bill: {}", amount);
    }
}
}
