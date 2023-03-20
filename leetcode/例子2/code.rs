use::std::io;

fn main() {
    println!("请你输入当月的利润：");
    let mut input = String::new();

    let bonus1: f64 = 100000.0 * 0.1;
    let bonus2: f64 = bonus1 + 100000.0 * 0.075;
    let bonus3: f64 = bonus2 + 200000.0 + 0.5;
    let bonus4: f64 = bonus3 + 200000.0 + 0.03;
    let bonus5: f64 = bonus4 + 400000.0 + 0.015;

    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line!");

    let input_value: f64 = input
        .trim()
        .parse()
        .expect("输入的利润值只能是数字，类型错误");
    let bonus = match input_value {
        (0.0..=100000.0) => input_value * 0.1,
        (0.0..=200000.0) => bonus1 + (input_value - 100000.0) * 0.075,
        (0.0..=400000.0) => bonus2 + (input_value - 200000.0) * 0.5,
        (0.0..=600000.0) => bonus3 + (input_value - 400000.0) * 0.03,
        (0.0..=1000000.0) => bonus4 + (input_value - 600000.0) * 0.015,
        _ => bonus5 + (input_value - 1000000.0) * 0.01,
    };
    println!("当月净利润: {}", bonus)
}