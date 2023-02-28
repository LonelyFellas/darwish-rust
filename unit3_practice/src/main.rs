use std::io;

fn main() {
    println!("Please input a number: ");
    let mut read_value = String::new();

    io::stdin().read_line(&mut read_value).expect("Fail to the number");
    let input_number: u32 = read_value.trim().parse().expect("Only input in number");

    let fic_number = fic_iterative(input_number);

    println!("f({}): {}",input_number,  fic_number)
}

fn fic_recursive(number: u32) -> u64 {
    match number {
        0 => 0,
        1 => 1,
        _ => fic_recursive(number - 1) + fic_recursive(number - 2)
    }
}

fn fic_iterative(number: u32) -> u64 {
    let mut n_0 = 0;
    let mut n_1 = 1;
    for _ in 0..number {
        let temp = n_0;
        n_0 = n_1;
        n_1 = temp + n_0;
    }

    n_0
}