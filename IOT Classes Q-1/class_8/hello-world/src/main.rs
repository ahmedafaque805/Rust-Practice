// #[derive(Debug)]
// struct Student{
//         first_name: String,
//         last_name: String,
//         age: i8,
//         single: bool,
//         height: f32,
//     }


// fn main() {
//     let student_01 = Student{
//         single: true,
//         first_name: "Afaque".to_string(),
//         last_name: "Ahmed".to_string(),
//         age: 17,
//         height: 5.8,
//     };
//     println!("{:?}", student_01);
// }


#[derive(Debug)]
struct Car{
        name: String,
        color: String,
        cc: i32,
        model: bool,
        Type: String,
    }


fn main() {
    let car_01 = Car{
        name: "mehran".to_string(),
        color: "Black".to_string(),
        cc: 1800,
        model: true,
        Type:"hatchback".to_string(),
    };
    println!("{:?}", car_01);
}









