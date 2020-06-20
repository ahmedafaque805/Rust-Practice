// #[derive(Debug)]
// enum EngineCC {
//     eight_hundred,
//     ten_hundred,
//     fifteen_hundred,
// }



// #[derive(Debug)]
// struct Vechile {
//     name:String,
//     color:String,
//     EngineCapacity: EngineCC,
// }


// #[derive(Debug)]
// enum shape {
//     Rectangle{x:f64,y:f64},
//     Circle(f64),
//     Square(f64),
// }


// impl Vechile{
//     fn Vechile_info(&self) {
//         println!("Vechile Name:{}\n Vechile Colour:{}\nVechile cc:{}",
//         self.name,
//         self.color,
//         match self.EngineCapacity{
//             EngineCC::eight_hundred => "800 CC",
//             EngineCC::ten_hundred => "1000 CC",
//             EngineCC::fifteen_hundred => "1500 CC",
            
//         }
//          );
//     }
// }


// impl shape {
//     fn area(shape:shape)->f64{
     
//     match shape{
//         shape::Rectangle{x,y} => 0.0 as f64,
//         shape::Circle{x} => 3.14*x*x as f64,
//         shape::Square(x) => x*x as f64,
//     }
//     }
// }




// /fn main() {
    
    //  let boss = EngineCC::eight_hundred;
    //  let dragonR = EngineCC::ten_hundred;
    //  let sr  = EngineCC::fifteen_hundred;

    // match sr {
    //     EngineCC::eight_hundred => print!("Mehran" ),
    //     EngineCC::ten_hundred => print!("DRagonR" ),
    //     EngineCC::fifteen_hundred => print!("Civic Turbo"),

    // }
    


// let Vechile_01 = Vechile{name: "Civic".to_string(),
// color: "Silver".to_string(),
// EngineCapacity: EngineCC::fifteen_hundred};
// Vechile_01.Vechile_info();

//// Also
//// println!("{:?}",Vechile_01 )



// let object = shape::Rectangle{x:23.0,y:32.0};
// let object = shape::Circle (24.0);
// match object {
//     shape::Rectangle{x,y} => println!("Area of Rectangle {}", x*y),
//     shape::Circle(x) => println!("Area of Circle {}",  3.14*x*x),
//     shape::Square(x) => println!("Area of Square {}",  x*x ),
// };

// println!("The area of shape cicle is {}",shape::area(object) );
// }







// fn divide (x:f64,y:f64)-> Option<f64>{
// if y == 0.0{
//     None
// }
// else {
//     Some(x/y)
// }
// }


// fn main() {
//     println!("{:?}",divide(24.0, 0.0).unwrap());
//     match divide(24.0, 0.0){
//         Some(x) => println!("the result is this: {}",x ),
//         None => println!("Invalid Divider"),
//     };
// }