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
  let strs = vec![String::from("flower"),String::from("flow"),String::from("flight")];
  println!("{:?}", longest_common_prefix(strs))
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
  // let mut min_len = strs[0].len();
  // let mut index = 0;
  // let mut count = 1;

  // loop {
  //   if min_len > strs[count].len() {
  //     min_len = strs[count].len();
  //     index = count;
  //   }

  //   if count >= strs.len() - 1 {
  //     break;
  //   }
  //   count += 1;
  // }
  // println!("{index}");

  // for c in strs[index].chars() {

  // }
  let a: String = strs.iter()
  .max()
  .unwrap()
  .chars()
  .zip(strs.iter().min().unwrap().chars())
  .take_while(|x| x.0 == x.1)
  .map(|x| x.0)
  .collect();
  println!("{:?}", a);

  return "--------hello".to_string();
}