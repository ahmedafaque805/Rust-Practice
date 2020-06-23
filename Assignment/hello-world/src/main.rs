                               //Assignment 1

// pub struct NewsArticle {
//     pub author: String,
//     pub content: String,
//  } 

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {}", self.content,self.author)
//     }
// } 

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//     format!("{} {}", self.username, self.content)
//     }
// }


// fn main() {
//     let my_article = NewsArticle {
//     author: String::from("Afaque Ahmed"),
//     content: String::from("Pakistan zindabad"),
//     };
    
//     println!("\nNew article available! {}", my_article.summarize());     


//     let my_tweet = Tweet {
//     username: String::from("Afaque Ahmed"),
//     content: String::from("About 'Happy live'"),
//     };
    
//     println!("new tweet by {}", my_tweet.summarize());  
// }










                                   //Assignment 2

// pub struct Book {
//     pub author: String,
//     pub name: String,
// }

// impl Book {
//     pub fn new(name: String,author: String) -> Book {
//     Book {
//         name,
//         author,
// }
// }
// }


// pub trait BookInformation {
//     fn info(&self) -> String;
// }



// impl BookInformation for Book {
//     fn info(&self) -> String {
//     format!("{} and Name is : {}", self.author, self.name)
//     }
// }



//  fn main() {
//     let my_book = Book {
//     author: String::from("Afaque Ahmed"),
//     name: String::from("Aziz"),
//     };
    
//     println!("\nAuthor is {}", my_book.info());     


// }






// mod alpha {
// #[derive(Debug)]
// pub struct Student {
//     name: String,
//     }
    
//     impl Student {
//     pub fn new(name: String) -> Student {
//     Student {
//         name,
//     }
//     }
//     }
//     }
    
    
// fn main () {
//     // let student01 = Student {
//     //     name: String::from("Areeb"),
//     // };
//     // println!("{:#?}",student01);
    
//     let student_02 = alpha::Student::new(String::from("Areeb"));
//     println!("{:#?}",student_02);      
// }






















                             //Assignment 3






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
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }


// fn main () {
//     let p1 = Pair::new(5,10);
//     println!("{:?}",p1);
//     p1.cmp_display();
// }





















                                 //Assignment: 4


// fn main () {
//     let two = |n,m| {
//         n+m
//     };
//     println!("Sum is {}",two(10,89));
// }



// fn main() {
//     let n = || {
//             3
//     };
//         if n() % 2 == 0 {
//             println!("false");
//         }
//         else {
//             println!("true");
//         }
       
// }

