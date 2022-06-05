use std::io;

fn main() {

    println!("Generate the nth Fibonacci number");
    println!("Enter value of n");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read value of n");
    
    let n: u32 = n.trim().parse().expect("Enter a positive number");

    println!("N: {}", n);

    let mut num1: u32 = 0;
    let mut num2: u32 = 1;
    let mut num3: u32;

    println!("{}",num1);
    println!("{}",num2);
    for _i in 0..n-2 {

        num3 = num1 + num2;
        println!("{}",num3);
        num1 = num2;
        num2 = num3;

    }

    




    
    //println!("Fibonacci Series");

}
