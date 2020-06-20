fn main() {
  //  println!("hello, world!");
   // println!("The sum is: {}",addTwoNumber(45,30));
    // println!("The square is: {}",sqr(25));
   // println!("the square is: {}",calculate_square());}
// addtwonumber function name
// (x:i32,y:i32) funstion parameters
// ->i32 indicates return type
// sum
//    fn sqr(x:i32)->i32{
//    let y = x*x ;
//    y
 // }

 use std::io;
// fn calculate_square()->i32{
    let mut input_01 = String::new();
    io::stdin().read_line(&mut input_01);
    let input_01:i32 = input_01.trim().parse().unwrap();
    
    let mut input_02 = String::new();
    io::stdin().read_line(&mut input_02);
    let input_02:i32 = input_02.trim().parse().unwrap();
//     input_01*input_02

// }
// fn take_ful_name()->(String,String){
//     println!();
//  let mut input_01 = String::new();
//     io::stdin().read_line(&mut input_01);
    
    
//      let mut input_02 = String::new();
//     io::stdin().read_line(&mut input_02)

// }
  

// }



fn arthimatic_operations(x:i32,y:i32)->(i32,i32,i32,i32){
    (x+y,x-y,x*y,x/y)
}
}