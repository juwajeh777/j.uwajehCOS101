// campus cafe order system

use std::io;

fn main() {
    loop {
    println!("CODE        ITEM          PRICE(N)");
    println!("T           Tea           800");
    println!("C           Coffee        1,200");
    println!("S           Sandwich      2,000");
    println!("J           Juice         1,500");

    let t:f32 = 800.0;
    let c:f32 = 1200.0;
    let s:f32 = 2000.0;
    let j:f32 = 1500.0;

    let mut input = String::new();
    println!("New customer order? (yes or exit): ");
    io::stdin().read_line(&mut input).expect("Not a valid input");
    input = input.trim().to_lowercase();
    if input == "exit" {
        break
    } else if input == "yes" {
        println!("Welcome, Place your order");
    } else {
        println!("Invalid response");
    }

    println!("Enter desired item code: ");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Not a valid input");
    let code = code.trim().to_uppercase();

    println!("Enter quantity wanted: ");
    let mut qnty = String::new();
    io::stdin().read_line(&mut qnty).expect("Not a valid input");
    let qnty:f32 = qnty.trim().parse().expect("Not a valid number");

    if code == "T" {
        let total_cost:f32 = t * qnty;
        if total_cost >= 5000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "C" {
        let total_cost:f32 = c * qnty;
        if total_cost >= 5000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "S" {
        let total_cost:f32 = s * qnty;
        if total_cost >= 5000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "J" {
        let total_cost:f32 = j * qnty;
        if total_cost >= 5000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    }
}
}