                                // CLass code
// #[derive(Debug)]
// struct Spiderman {
//     name: String,
// } //Type

// #[derive(Debug)]
// struct Superman {
//     name: String,
// } //Type

// #[derive(Debug)]
// struct Batman {
//     name: String,
// } //Type

// #[derive(Debug)]
// struct Hulk {
//     name: String,
// } //Type


// pub trait Power {
//     fn power_score(&self)->i32 {
//         50
//     }
// } //Trait


// impl Power for Superman {
//     fn power_score(&self) -> i32 {
//         100
//     }
// } //Implemented trait for type Superman


// impl Power for Spiderman {
//     fn power_score(&self) -> i32 {
//         80
//     }
// } //Implemented trait for type Spiderrman


// impl Power for Batman {
//     fn power_score(&self) -> i32 {
//         50
//     }
// } //Implemented trait for type Batman

// impl Power for Hulk {
//     fn power_score(&self) -> i32 {
//         50
//     }
// } //Implemented trait for type Spiderrman





// fn main () {
//     let my_spiderman = Spiderman {
//         name: String::from("MYSPIDERMAN"),
//     }; //my_spiderman is the instance Spiderman

//     let my_superman = Superman {
//         name: String::from("MYSUPERMAN"),
//     };//my_superman is the instance Superman
    
//     let my_batman = Batman {
//         name: String::from("MYBATMAN"),
//     };//my_batman is the instance Batman
    
//     let my_hulk = Hulk {
//         name: String::from("HULK"),
//     };//my_hulk is the instance Hulk
    
//     println!("{}",my_spiderman.power_score());  //calling method
//     println!("{}",my_superman.power_score());  //calling method
//     println!("{}",my_batman.power_score());   //calling method
//     println!("{}",my_hulk.power_score());    //calling method

// }






                               //HOME Practice




// #[derive(Debug)]
// struct Point<T,U>{
//     x:T,
//     y:U
// }

// impl<T,U> Point<T,U> {

//     fn mixup<V,W>(self, other:Point<V,W>)-> Point<T,W> {
        
//         Point{
//             x: self.x,
//             y: other.y
//         }
//     }
// }


// fn main() {

//      let p1 = Point{
//          x:3,
//          y:'a'
//      };

//      let p2 = Point{
//          x:6.74,
//          y:"a string"
//      };

//     let p3 = p1.mixup(p2);
//     println!("{:?}",p3);
// }


// p1 = (3,'a');
// p2 = (6.74,"a string")

// p3 = (3,"a string")








// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }




// fn main() {
//     let school = vec![20,35,13,100,30];
//     let mut class = school[3];

//     for n in school {
//        if n > class{
//             class = n;
//         }
//     }
//     println!("{:?}",class)
// }






// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//    assert_eq!(result, 100);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }





// #[derive(Debug)]

// struct Point <T,U> {
//     x:T,
//     y:U
// }

// fn main() {
//     let integers = Point { x: 15 , y: 70  };
//     let float = Point { x: 18.56 , y: 9.2 };

//    println!("{:?} {:?}",integers,float)
// }




