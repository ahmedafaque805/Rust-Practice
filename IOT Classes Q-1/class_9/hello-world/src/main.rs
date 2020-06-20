
// // fn main() {
// //     let t:(i32,i32,i32) = (23,43,54);
// //     let s = (t.0+t.1+t.2) /2 ;
// //     let r = s*(s-t.0)*(s-t.1)*(s-t.2);
// //     let final_value = s.sqrt();
// //     println!("the result is {},{},{}", s,r,final_value);
// // }


// // #[derive(Debug)]
// // struct Points {
// //     a:i32,
// //     b:i32,
// //     c:i32
// // }


// // impl Points {
// //     fn calculate_value(self) -> i32 {
// //         let s = {self.a + self.b + self.c}/2;
// //         s
// //     }
// // }

// // fn main() {
// //     let point_01 = Points{a:23,b:43,c:54};
// //     println!("{}", point_01.calculate_value());
// // }




// // #[derive(Debug)]
// // struct Area {
// //     a:i32,
// //     b:i32,
// //     c:i32
// // }


// // impl Area {
// //     fn calculate_area(self) -> i32 {
// //         let s = {self.a + self.b + self.c}/2;
// //         s
// //     }
// // }

// // fn main() {
// //     let area_01 = Area{a:23,b:43,c:54};
// //     let area_02 = Area{a:10,b:40,c:62};
// //     println!("{}", area_02.calculate_area());
// // }

// // methode 
// //a function which having the self parameter is known as method






// #[derive(Debug)]
// struct Area_of_rectangle {
//     length:i32,
//     width:i32,
// }


// impl Area_of_rectangle {
//     fn calculate_area_of_rectangele(self) -> i32 {
//         let s = {self.length*self.width};
//         s
//     }
// }

// fn main() {
//     let area_01 = Area_of_rectangle{length:10,width:20};
//     println!("{}", area_01.calculate_area_of_rectangele());
// }




// mod student;
// fn main() {
//     student::student_main();
// }

// mod student;
// use student::student_work;
// fn main() {

// }



mod vehicle;
fm main() {
    let spec = vehicle_spec{name:Mehran,companyname:Suzuki,color:White,transmission:xyz,model:2018}
    println!("{}",spec.vehicle_spec());
}
