// const ARR_CHARACTER_PARDON: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
use std::io;
fn main() {
    println!("Please input a roman's number: ");
    let mut roman_number = String::new();
    io::stdin()
        .read_line(&mut roman_number)
        .expect("Failed read to the roman' number");
    roman_number = roman_number.trim().parse().expect("Failed!");

    let reset_character = reset_roman_number(roman_number);

    let mut sum = 0;
    for char_number in reset_character.chars() {
        let even_number = match char_number {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            'a' => 4,
            'b' => 9,
            'c' => 40,
            'd' => 90,
            'e' => 400,
            'f' => 9000,
            _ => panic!("Only allow to roman number")
        };

        sum += even_number
    }


    println!("{}", sum);
}

fn reset_roman_number(mut roman_number: String) -> String {
    roman_number = roman_number.replace("IV", "a");
    roman_number = roman_number.replace("IX", "b");
    roman_number = roman_number.replace("XL", "c");
    roman_number = roman_number.replace("XC", "d");
    roman_number = roman_number.replace("CD", "e");
    roman_number = roman_number.replace("CM", "f");
    roman_number
}
