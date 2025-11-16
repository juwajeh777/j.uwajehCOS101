use std::io;
use std::f64::consts::PI;

fn main() {
    println!("         CALCULATOR FOR SHAPES            ");
    println!("---------------------------------------");
    println!("    What do you want to calculate?   ");
    println!("--------------------------------------");
    println!("      1. Area of Trapezium"); 
    println!("      2. Area of Rhombus"); 
    println!("      3. Area of Parallelogram"); 
    println!("      4. Area of Cube"); 
    println!("      5. Volume of Cylinder"); 

    

    let choice = read_input("Please input your choice (1-5):").trim().parse::<u32>().unwrap();

    if choice == 1 {

    

       let h = read_number("Input the height:");
       let b1 = read_number("Input base1:");
       let b2 = read_number("Input base2:");

    

       println!("Area of Trampezium is: {}", area_trapezium(h, b1, b2)); 

    }

    else if choice == 2 {
        let d1 = read_number("Input diagonal1:");
        let d2 = read_number("Input diagonal2:");

    

        println!("Area of Rhombus is: {}", area_rhombus(d1, d2));
    }

    else if choice == 3 {
        let base = read_number("Input length of base:");
        let alt = read_number("Input altitude height:");
        println!("Area of Parallelogram is: {}", area_parallelogram(base, alt));
    }

    else if choice == 4 {
        let side = read_number("Input length of side:");
        println!("Surface Area of Cube is: {}", area_cube(side));
    }

    else if choice == 5 {
        let r = read_number("Input radius of cylinder:");
        let h = read_number("Input height of cylinder:");
        println!("Volume of cylinder is: {}", volume_cylinder(r, h));
    }

    else {
        println!("Invalid choice");
    }

    fn read_input(prompt:&str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input // or return input;
    }

    fn read_number(prompt:&str) -> f64 {
        read_input(prompt).trim().parse::<f64>().unwrap()
    }

    fn area_trapezium(h:f64, b1:f64, b2:f64) -> f64 {
        (h / 2.0) * (b1 + b2)
    }
    fn area_rhombus(d1:f64, d2:f64) -> f64 {
        0.5 * d1 * d2
    }
    fn area_parallelogram(base:f64, alt:f64) -> f64 {
        base * alt
    }
    fn area_cube(side:f64) -> f64 {
        6.0 * side * side
    }
    fn volume_cylinder(r:f64, h:f64) -> f64 {
        PI * r * r * h
    }

}