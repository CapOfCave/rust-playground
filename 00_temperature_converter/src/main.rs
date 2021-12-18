use std::io;

fn main() {
    loop {
        println!("Enter 1 to convert Fahrenheit to Celsius, 2 to convert Celsius to Fahrenheit or anything else to exit the program.");
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Error while trying to read line.");

        match operation.trim().parse() {
            Ok(1) => {
                let unconverted_value = get_unconverted_value("Fahrenheit");
                let result = convert_to_celsius(unconverted_value);
                println!("{} °F is equal to {} °C.", unconverted_value, result);
            }
            Ok(2) => {
                let unconverted_value = get_unconverted_value("Celsius");
                let result = convert_to_fahrenheit(unconverted_value);
                println!("{} °C is equal to {} °F.", unconverted_value, result);
            }
            Ok(_) => break,
            Err(_) => break,
        };
    }
    println!("Goodbye.");
}

fn get_unconverted_value(expected_type: &str) -> f64 {
    let mut unconverted_value = String::new();

    println!("Enter the temperature in {} to convert:", expected_type);

    io::stdin()
        .read_line(&mut unconverted_value)
        .expect("Error while trying to read line.");

    unconverted_value
        .trim()
        .parse()
        .expect("Please enter a valid decimal number.")
}

fn convert_to_fahrenheit(temperature_celsius: f64) -> f64 {
    temperature_celsius * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(temperature_fahrenheit: f64) -> f64 {
    (temperature_fahrenheit - 32.0) * 5.0 / 9.0
}
