use std::io;

enum TemperatureMode {
    FahrenheitToCelsius,
    CelsiusToFahrenheit
}

fn main() {
    println!("Please input convert mode:");
    println!("1 -> it is means what a fahrenheit to celsius:");
    println!("2 -> it is means what a celsius to fahrenheit:");

    let mut mode_number = String::new();
    let mode_number1 = read_input_number(&mut mode_number).parse().expect("The mode_number's type is error");

    match mode_number1 {
        1 => fahrenheit_to_change_celsius(),
        2=> celsius_to_change_fahrenheit(),
        _ => panic!("You input a invalid number of mode")
    };


}
/**
 * Rust input a number
 */
fn read_input_number(number: &mut String) -> String {
    io::stdin().read_line(number).expect("Fail to the number");

    number.trim().to_string()
} 
fn fahrenheit_to_change_celsius() {
    println!("Please input a value of fahrenheit:");
    let mut number = String::new();
    let fah = read_input_number(&mut number).parse().expect("The fahrenheit_number's type is error");

    let cel_number = (&fah - 32.0) / 1.8;
    println!("The {} fahrenheit equal {:.2} celsius", fah, cel_number);
}

fn celsius_to_change_fahrenheit() {
    println!("Please input a value of celsius:");
    let mut number = String::new();
    let cel = read_input_number(&mut number).parse().expect("The celsius_number's type is error");

    let fah_number = &cel * 1.8 + 32.0;
    println!("The {} fahrenheit equal {:.2} celsius", cel, fah_number);
}