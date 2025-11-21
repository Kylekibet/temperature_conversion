use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");

    let degree_sign = '\u{00B0}';
    let f_temp = get_input();

    loop {
        println!("Would you like to convert to \n1. Fahrenheit \n2. Celsius");

        let choice = get_input();

        match choice as u32 {
            1 => {
                println!("{:.2}{degree_sign}F", c_to_f(f_temp));
                break;
            }
            2 => {
                println!("{:.2}{degree_sign}C", f_to_c(f_temp));
                break;
            }
            _ => println!("Please choose between 1 and 2"),
        }
    }
}

fn f_to_c(num: f64) -> f64 {
    (num - 32.0) / 1.8
}

fn c_to_f(num: f64) -> f64 {
    (num * 1.8) + 32.0
}

fn get_input() -> f64 {
    loop {
        let mut number = String::new();

        println!("Please input a number:");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid floating point number.");
                continue;
            }
        };
        return number;
    }
}
