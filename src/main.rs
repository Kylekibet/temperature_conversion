use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");
    let degree_sign = '\u{00B0}';

    let (temp_to_convert, unit) = get_temperature_and_unit();

    if unit == 'C' {
        let converted_temp = c_to_f(temp_to_convert);
        println!(
            "{:.2}{degree_sign}C is {:.2}{degree_sign}F",
            temp_to_convert, converted_temp
        );
    } else {
        // unit is 'F'
        let converted_temp = f_to_c(temp_to_convert);
        println!(
            "{:.2}{degree_sign}F is {:.2}{degree_sign}C",
            temp_to_convert, converted_temp
        );
    }
}

fn f_to_c(num: f64) -> f64 {
    (num - 32.0) / 1.8
}

fn c_to_f(num: f64) -> f64 {
    (num * 1.8) + 32.0
}

fn get_temperature_and_unit() -> (f64, char) {
    loop {
        println!("Please enter the temperature to convert (e.g., 80C, 100F):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            println!("Invalid input. Please try again.");

            continue;
        }

        let unit = input.chars().last().unwrap().to_ascii_uppercase();

        let number_part = &input[..input.len() - 1];

        if unit != 'C' && unit != 'F' {
            println!("Invalid format. Please use 'C' or 'F' for the unit (e.g., 80C, 100F).");

            continue;
        }

        let number: f64 = match number_part.trim().parse() {
            Ok(num) => num,

            Err(_) => {
                println!(
                    "Invalid number. Please enter a valid floating point number followed by 'C' or 'F'."
                );

                continue;
            }
        };

        return (number, unit);
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_f_to_c() {
        assert_eq!(f_to_c(32.0), 0.0);

        assert_eq!(f_to_c(212.0), 100.0);
    }

    #[test]

    fn test_c_to_f() {
        assert_eq!(c_to_f(0.0), 32.0);

        assert_eq!(c_to_f(100.0), 212.0);
    }
}
