// fn add_two_number<'a,'b>(x:&'a mut i32, y:&'b i32)-> &'a i32 {
//     *x = *x + y;
//     x 
// }

// fn main() {
//     println!("the is this:{}",add_two_number(&mut 20,&76));
//     let sum_c = |num01: i32,num02:i32 |->i32{
//        num01+num02
//     };
// // Sum by Function
//      fn sum_f(num01: i32, num02: i32)-> i32 {
//          num01+num02
//      }

//     println!("Sum by clousre:{}",sum_c(34,54));
//     println!("Sum by clousre:{}",sum_f(24,74));


//     let pi = 3.14;
//     let area_circle = |radius: f64|->f64 {
//         pi* (radius*radius)
//     };
//     fn area_circle(radius:f64)-> f64 {
//         3.14* (radius*radius)
//     }

//     println!("Area of circle by Clousre: {}",area_circle(3.0));
//     println!("Area of circle by Function: {}",area_circle(3.0));
// }








use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// App code
fn generate_workout (intensity: u32) {

if intensity < 25 {
    println!("Do {} pushups",simulated_expensive_calculation(intensity));
    println!("Do {} situps",simulated_expensive_calculation(intensity))
}
else {
    println!("run {} kms", simulated_expensive_calculation(intensity))
}
}

fn main () {
    generate_workout(20);
}


