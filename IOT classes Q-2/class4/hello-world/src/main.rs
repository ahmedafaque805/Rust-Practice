// use std::fmt::Display;
// #[derive(Debug)]
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn compare(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// use std::collections::HashMap;

// fn main () {
//     let p1 = Pair::new(5,10);
//     println!("{:?}",p1);
//     p1.compare();

//     let mut h1 = HashMap::new();
//     h1.insert("Blue",5);

//     let mut h2 = HashMap::new();
//     h2.insert("Blue",10);

//     let p1 = Pair::new(h1,h2);
//     println!("{:#?}",p1);
// }









// fn average(list: &[i32]) -> i32 {
//     let mut divide = list[0];

//     for &item in list.iter() {
//         if item > divide {
//             divide = item;
//         }
//     }

//     divide
// }

// fn main() {
//     let number_list = vec![3, 7, 5, 13, 20, 23 ,39, 23, 56, 40, 14, 12, 23, 29];
//     let result = average(&number_list);
//     println!("The largest number is {}", result);
// }


// fn main () {
//     let arrint = [3, 7, 5, 13, 20, 23 ,39, 23, 56, 40, 14, 23, 12, 23, 29];
//     let _count = arrint.len();
//     let mut sum = 0;
//     for i in 0.._count{
//         sum+= arrint[i];
//     }

//     println!("the average of number is: {} ",sum/_count);

//     println!("the average of number with function  is: {} ",average(&arrint));
 
//     println!("the average of number with function  is: {} ",);

// }



// fn average(arrint:&[usize])-> usize{
//     let _count = arrint.len();
//     let mut sum = 0;
//     for i in 0.._count{
//         sum+= arrint[i];
//     }
//     sum/_count
// }






// use std::ops::Add;
// use std::ops::Div;
// fn average1 <T> (list: &[T]) -> T
// where T: Add<Output=T> + Div<Output=T> + Copy + Clone + From<i32>
// {
//     let mut sum = list[0];
//     let count = T::from(list.len() as i32);
//     for x in 0..list.len() {
//         sum = sum + list[x];
//     }
//     sum/count
// }


// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = average1(&number_list);
//     println!("The largest number is {}", result);
    
//     let number_list_1 = vec![109, 40, 29, 10, 65];
//     let result = average1(&number_list_1);
//     println!("The largest number is {}", result);
    
// }








// fn main () {
//     let string1 = "ABCD";
//     let string2 = "FUYKf";
//     let result = lifetime(string1,string2);
//     println!("the string is {}", result); 
// }

// fn lifetime<'a>(_x: &'a str, mut _y: &'a str) -> &'a str {
//     {
//         let x = "Afaque";
//         _y = &x;
//     }
//     _y
// }






// fn main () {
//     let return_two = |num| {
//         num+2.5
//     };
//     let result = return_two(6.8);
//     println!("Sum is {}",result);
// }


// fn main () {
//     let num1 = || {
//         6
//     };
//     let num2 = || {
//         7
//     };
//     let result = num1()+num2();
//     println!("Sum is {}",result);
// }


fn main () {
    let two = |n,m| {
        n+m
    };
    println!("Sum is {}",two(6,7));
}



// use std::thread;
// use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn main () {
//     let result = simulated_expensive_calculation(20);
//     println!("{}",result)
// }


// // App code
// fn generate_workout (intensity: u32) {

// if intensity < 30 {
//     println!("Do {} pushups",simulated_expensive_calculation(intensity));
//     println!("Do {} situps",simulated_expensive_calculation(intensity))
// }
// else {
//     println!("run {} kms", simulated_expensive_calculation(intensity))
// }
// }






