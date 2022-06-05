use std::io;

fn main() {
    println!("Temperature Conversion");
    println!("1 -> Fahrenheit to Celsius");
    println!("2 -> Celsius to Fahrenheit");
    println!("Enter your choice");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("Please type a number");

    println!("Your choice: {}", choice);


    println!("Enter temperature: ");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = temperature.trim().parse().expect("Please provide a valid temperature");

    match choice {
        1 => {
            println!("Temperature in Celsius: {}", (temperature-32.0)*5.0/9.0);
        },
        2 => {
            println!("Temperature in Fahrenheit: {}", (temperature*9.0/5.0)+32.0);
        },
        _ => println!("Invalid choice")
    }


    
        



}
