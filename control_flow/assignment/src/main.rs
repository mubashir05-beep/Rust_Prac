use std::io;

fn main() {
    println!("Enter Temperature:");

    let mut input_temperature = String::new();
    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read!");

    let input_temperature: f32 = input_temperature
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    println!("Please choose an option:");
    println!("1) Convert to Celsius (Provide temp in Fahrenheit).");
    println!("2) Convert to Fahrenheit (Provide temp in Celsius).");

    let mut user_option = String::new();
    io::stdin()
        .read_line(&mut user_option)
        .expect("Failed to read user input!");

    let user_option: u32 = user_option
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    match user_option {
        1 => {
            let converted_temp = fahrenheit_to_celsius(input_temperature);
            println!("Temp in Celsius: {}", converted_temp);
        }
        2 => {
            let converted_temp = celsius_to_fahrenheit(input_temperature);
            println!("Temp in Fahrenheit: {}", converted_temp);
        }
        _ => println!("Invalid option. Please select 1 or 2."),
    }
}

fn fahrenheit_to_celsius(temp_fahrenheit: f32) -> f32 {
    (temp_fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp_celsius: f32) -> f32 {
    (temp_celsius * 9.0 / 5.0) + 32.0
}
