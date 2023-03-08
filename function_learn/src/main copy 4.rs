// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50};


//     assert_eq!(rect1.area(), 1500);
// }


// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(&self) {
//         println!("the current state is {}", self.color);
//     }
// }


// fn main() {
//     let light = TrafficLight {
//         color: "red".to_owned(),
//     };

//     light.show_state();

//     println!("{:?}", light);
// }


// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(self) {
//         println!("the current state is {}", self.color);
//     }

//     pub fn change_state(self: &mut Self) {
//         self.color = "green".toString();
//     }
// }

// fn main() {

// }

// #[derive(Debug)]

// struct TrafficLight {
//     color: String,
// }
// impl TrafficLight {
//     pub fn new() -> TrafficLight {
//         TrafficLight {
//             color: String::from("red")
//         }
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }


// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
// }


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.width > other.height
//     }
// }

// impl Rectangle {

// }

// fn main() {

// }

// #[derive(Debug)]
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green
// }

// impl TrafficLight {
//     fn color (&self) -> &str {
//         match self {
//             TrafficLight::Red => "red",
//             TrafficLight::Yellow => "yellow",
//             TrafficLight::Green => "green"
//         }
//     }

// }

// fn main() {
//     let c = TrafficLight::Yellow;
//     assert_eq!(c.color(), "yellow");
//     println!("{:?}", c);
// }



// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 64];
    
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     println!("The largest char is {}", result);

// }


// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
//     fn y(&self) -> &T {
//         &self.y
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     let p1 = Point { x: 10.20, y: 20.40 };

//     println!("p.x = {}", p.x());
//     println!("p.y = {}", p.y());

//     println!("p1.y = {}", p1.x());
//     println!("p1.y = {}", p1.y());
// }


// struct Point<T, U> {
//     x: T,
//     y: U,
// }


// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point { x: self.x, y: other.y }
//     }
// }


// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };
    
//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
//     println!("{:?}", arr);
// }

// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(&arr);

    
// }

// struct A;
// struct S<A>;
// struct SGen<T>(T);

// fn reg_fn(_s: i32) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // 使用非泛型函数
//     reg_fn(3);          // 具体的类型
//     gen_spec_t(reg_fn(3));   // 隐式地指定类型参数  `A`.
//     gen_spec_i32(reg_fn(3)); // 隐式地指定类型参数`i32`.

//     // 显式地指定类型参数 `char`
//     generic::<char>('c');

//     // 隐式地指定类型参数 `char`.
//     generic('c');
// }

// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }


// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }


// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<S, W>(self, other: Point<S, W>) -> Point<T, W> {
//         Point { x: self.x, y: other.y}
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: "Middle"};

//     let p3 = p1.mixup(p2);
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, "Middle");
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }


// impl<T: std::ops:Sqrt<Output = T>> Point<T> {
//     fn distance_from_origin(&self) -> T {
//         (self.x.powi(2) + self.y.powi(2).sqrt())
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("{}", p.distance_from_origin())
// }
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }


// 修复错误
// struct Array<T, const N: usize> {
//     data : [T; N]
// }

// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1.0, 2.0, 3.0],
//         },
//         Array {
//             data: [1, 2]
//         }
//     ];
// }

