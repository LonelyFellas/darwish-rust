use std::io;

fn main() {
    println!("请输入想转化的摄氏度：");
    let mut cel = String::new();

    io::stdin()
        .read_line(&mut cel)
        .expect("输入摄氏度失败");

    let cel_number1: f32 = cel.trim().parse().expect("输入的类型的错误"); 


    let fah_number2 = fahrenheit_to_change_celsius(cel_number1);

    println!("摄氏度{cel}转化华氏度是：{fah_number2}");

    println!("请输入想转化的华氏度：");
    let mut fah = String::new();

    io::stdin()
        .read_line(&mut fah)
        .expect("输入华氏度失败");

    let fah_number1: f32 = fah.trim().parse().expect("输入的类型的错误"); 


    let cel_number2 = celsius_to_change_fahrenheit(fah_number1);

    println!("华氏度{cel}转化摄氏度是：{cel_number2}");
}

fn fahrenheit_to_change_celsius(cel_number: f32) -> f32 {
    cel_number * 1.8 + 32.0
}

fn celsius_to_change_fahrenheit(fah_number: f32) -> f32 {
    (fah_number - 32.0) / 1.8
}