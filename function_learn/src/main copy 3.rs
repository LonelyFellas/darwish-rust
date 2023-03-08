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

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}

impl TrafficLight {
    fn color (&self) -> &str {
        match self {
            TrafficLight::Red => "red",
            TrafficLight::Yellow => "yellow",
            TrafficLight::Green => "green"
        }
    }

}

fn main() {
    let c = TrafficLight::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);
}