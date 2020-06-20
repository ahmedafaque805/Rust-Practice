fn main() {
    let x = 10;
    let y = x;
    println!("{} ,{}",x,y);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    let s3 = &s1;
    let s4 = &s1;
    println!("{}, {}, {}, {}", s1, s2, s3, s4); 


}