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
  let nums: Vec<i32> = vec![10,4,8,3];

  let mut vector_temp: Vec<i32> = vec![];
  let mut left_sum = 0;
  let mut right_sum = 0;
  let mut num_sum = 0;
  for v in &nums {
    num_sum += v;
  }

  for (i, v) in nums.iter().enumerate() {
    if !matches!(i, 0) {
      left_sum += nums[i - 1];
    }
    num_sum -=  v;
    right_sum = num_sum;

    vector_temp.push((left_sum - right_sum).abs())
  }

  println!("{:?}", vector_temp)
}
