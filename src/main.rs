use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");

    let degree_sign = '\u{00B0}';
    let temp_to_convert = get_temperature_input();

    loop {
        println!("Would you like to convert to \n1. Fahrenheit \n2. Celsius");

        let choice = get_menu_choice();

        match choice {
            1 => {
                let converted_temp = c_to_f(temp_to_convert);
                println!("{:.2}{degree_sign}C is {:.2}{degree_sign}F", temp_to_convert, converted_temp);
                break;
            }
            2 => {
                let converted_temp = f_to_c(temp_to_convert);
                println!("{:.2}{degree_sign}F is {:.2}{degree_sign}C", temp_to_convert, converted_temp);
                break;
            }
            _ => println!("Please choose between 1 and 2")
        }
    }
}

fn f_to_c(num: f64) -> f64 {
    (num - 32.0) / 1.8
}

fn c_to_f(num: f64) -> f64 {
    (num * 1.8) + 32.0
}

fn get_temperature_input() -> f64 {
    loop {
        let mut number = String::new();

        println!("Please enter the temperature to convert:");

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

fn get_menu_choice() -> u32 {
    loop {
        let mut choice = String::new();

        println!("Please enter your choice (1 or 2):");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter 1 or 2.");
                continue;
            }
        };
    }
}
