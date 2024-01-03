use std::io;

fn main() {
    println!("Enter Day Number:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read!");

    let user_input: u32 = user_input.trim().parse().expect("Input was not a number!");

    if user_input == 1 {
        println!("Monday");
    } else if user_input == 2 {
        println!("Tuesday");
    } else if user_input == 3 {
        println!("Wednesday");
    } else if user_input == 4 {
        println!("Thursday");
    } else if user_input == 5 {
        println!("Friday");
    } else if user_input == 6 {
        println!("Saturday");
    } else if user_input == 7 {
        println!("Sunday");
    } else {
        println!("Invalid day number");
    }

    let mut _count=0;
    'counting_up:loop {
        println!("count = {_count}");
        let mut _remaining = 10;
        loop{
            println!("remaining={_remaining}");
            if _remaining==9{
                break;
            }
            if _count==2{
                break 'counting_up;
            }
            _remaining-=1;
        }
        _count += 1;

    }
    println!("End count = {_count}");

    
}
