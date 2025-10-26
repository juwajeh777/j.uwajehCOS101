// rust program to determine the annual incentive of an employee

use std::io;

fn main() {
    println!("Is the individual experienced? input 'Yes' if experienced or 'No' if inexperienced" );
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input1:String= input1.trim().parse().expect("Not a valid input");

    println!("Enter the age of the employee: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Not a valid input");
    let age:i32 =age.trim().parse().expect("Not a valid number");

    if input1.trim() == "Yes" && age >= 40 {
        println!("Employee incentive is N1,560,000");
    } else if input1.trim() == "Yes" && age >= 30 {
        println!("Employee incentive is N1,480,000");
    } else if input1.trim() == "Yes" && age < 28 {
        println!("Employee Incentive is N1,300,000");
    } else if input1.trim() == "No" {
        println!("Employee incentive is N100,000");
    } else {
        println!("Invalid");
    }
}