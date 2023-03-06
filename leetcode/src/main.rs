// fn main () {
//     let vector: Vec<String> = vec!["++X".to_string(), "X++".to_string(), "--X".to_string(), "X++".to_string()];

//     for v in vector {
//         let operate = match v.as_str() {
//             "++X" => 1,
//             "X++" => 1,
//             "--X" => -1,
//             "X--" => -1,
//             _ => 0,
//         };

//         initValue += operate;
//     }

// }
fn main() {
  let mut vector: Vec<i32> = vec![1, 2, 3, 4, 0, 5, 0, 9, 0, 10, 11];

  let mut count = 0;
  for (i , &v) in vector.clone().iter().enumerate() {
    if matches!(v, 0) {
      vector.remove(i - count);
      vector.push(v as i32);
      count += 1;
    }

  }

  println!("{:?}", vector)
}
