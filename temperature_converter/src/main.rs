use std::io;

fn main() {

    println!("Enter 1 to convert Fahrenheit to Celsius, 2 to convert Celsius to Fahrenheit or 0 to exit the program.");
    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Error while trying to read line.");
    
    let operation: u8 = operation.trim().parse().expect("Please type a number");

    let mut unconverted_value = String::new();
    
    println!("Enter the temperature value to convert:");

    io::stdin()
        .read_line(&mut unconverted_value)
        .expect("Error while trying to read line.");

    let unconverted_value: f64 = unconverted_value.trim().parse().expect("Please enter a valid decimal number.");

    let result = if operation == 1 {
        convert_to_celsius(unconverted_value)
    } else if operation == 2 {
        convert_to_fahrenheit(unconverted_value)
    } else {
        0.0
    };

    println!("Your result is {}", result);

}

fn convert_to_fahrenheit(temperature_celsius: f64) -> f64 {
    temperature_celsius * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(temperature_fahrenheit: f64) -> f64 {
    (temperature_fahrenheit - 32.0) * 5.0 / 9.0
}
