fn main() {
    let mut count = 0;
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                if i != j && j != k && k != i {
                    println!("{i},{j},{k}");
                    count += 1;
                }
            }
        }
    }

    println!("count: {count}");
}
