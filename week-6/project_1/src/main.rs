// Program for food orders

use std::io;

fn main() {
    loop {
    println!("======================-----MENU----===========================");
    println!(" ITEM CODE             ITEM                        PRICE(N)");
    println!("   P           Poundo Yam/Edinkaiko Soup             3,200  ");
    println!("   F            Fried Rice & Chicken                 3,000  ");
    println!("   A            Amala & Ewedu Soup                   2,500   ");
    println!("   E             Eba & Egusi Soup                    2,000   ");
    println!("   W             White Rice & Stew                   2,500  ");

    let p:f32 = 3200.0;
    let f:f32 = 3000.0;
    let a:f32 = 2500.0;
    let e:f32 = 2000.0;
    let w:f32 = 2500.0;

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

    if code == "P" {
        let total_cost:f32 = p * qnty;
        if total_cost >= 10000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "F" {
        let total_cost:f32 = f * qnty;
        if total_cost >= 10000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "A" {
        let total_cost:f32 = a * qnty;
        if total_cost >= 10000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "E" {
        let total_cost:f32 = e * qnty;
        if total_cost >= 10000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    } else if code == "W" {
        let total_cost:f32 = w * qnty;
        if total_cost >= 10000.0 {
            let discount = total_cost * (1.0 - 0.05);
            println!("Final amount is: {} (5% discount applied)", discount);
        } else {
            println!("Final amount is: {} (No discount applied)", total_cost);
        }
    }
}
}