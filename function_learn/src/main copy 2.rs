// struct Point {
//     x: f64,
//     y: f64
// }

// impl Point {
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }

//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y}
//     }

// }

// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Rectangle {
//     fn area(&self) -> f64 {
//         let Point { x: x1, y: y1} = self.p1;
//         let Point { x: x2, y: y2} = self.p2;

//         (x1 - y2) * (y1 - y2).abs()
//     }

//     fn perimeter(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2} = self.p2;


//         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//     }


//     fn translate(&mut self, x: f64, y: f64) {
//         self.p1.x += x;
//         self.p2.x += x;

//         self.p1.y += y;
//         self.p2.y += y;
//     }

// }

// struct Pair(Box<i32>, Box<i32>);
// impl Pair {
//     fn destroy(self) {
//         let Pair(first, second) = self;

//         println!("Destroying pair({}, {})", first, second);
//     }
// }

// fn main () {
//     let rectangle = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0)
//     };


//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());

//     let mut square = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0)
//     };


//     square.translate(1.0, 1.0);

//     let pair = Pair(Box::new(1), Box::new(2));

//     pair.destroy();
// }

// // #[derive(Debug)]
// // struct Circle {
// //     x: f64,
// //     y: f64,
// //     radius: f64,
// // }

// // impl Circle {
// //     fn new(x: f64, y: f64, radius: f64) -> Circle {
// //         Circle {
// //             x: x,
// //             y: y,
// //             radius: radius,
// //         }
// //     }

// //     fn area(&self) -> f64 {
// //         std::f64::consts::PI * (self.radius * self.radius)
// //     }
// // }

// // fn main() {
// //     let circle = Circle::new(3.0, 3.0, 10.0);
// //     println!("{:?}", circle.area())
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // impl Rectangle {
// //     fn area(&self) -> u32 {
// //         self.width * self.height
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle { width: 30, height: 50 };

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         rect1.area()
// //     )
// // }

// pub struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     pub fn new(width: u32, height: u32) -> Self {
//         Rectangle { width, height }
//     }

//     pub fn width(&self) -> u32 {
//         return self.width;
//     }
// }

// fn main() {
//     let rect1 = Rectangle::new(30, 50);
//     println!("{}", rect1.width)
// }