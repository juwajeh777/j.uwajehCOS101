// rust program to find roots of quadratic equation

use std::io;

fn main() {
    print!("Quadratic equation Solver: ");

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Input value for a that is not equal to zero: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Input value for b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    println!("Input value for c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Not a valid number");

    // Find the discriminant d
    let d = b.powi(2) - 4.0 * a * c;

    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Roots of the equation are: {}, {}", x1,x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("Root of the equation is {}", x);
    } else if d < 0.0 {
        println!("Equation possesses no real roots");
    }


}