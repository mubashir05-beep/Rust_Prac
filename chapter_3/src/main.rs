use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    
    println!("Enter an Array Index Number:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number!");
    
    let element = a.get(index);
    
    match element {
        Some(value) => println!("The Value of the element at index {} is: {}", index, value),
        None => println!("Invalid index: {}", index),
    }

    let name = String::from("John");
    let greet = String::from("Mubahsir");
    let result = write_name(&greet);
    
    println!("{}", result);
}

fn write_name(name: &str) -> String {
    format!("Hello, {}", name)
}

