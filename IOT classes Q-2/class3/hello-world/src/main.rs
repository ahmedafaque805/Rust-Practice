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


// fn notify (item: impl Summary) {
// println!("{}",item.summarize());
// }



// fn main() {
//     let article_1 = NewsArticle {
//     author: String::from("Afaque Ahmed"),
//     content: String::from("Pakistan zindabad"),
//     };


//  //   println!("\nNew article available! {}", my_article.summarize());     


//     let tweet_1 = Tweet {
//     username: String::from("Afaque Ahmed"),
//     content: String::from("About 'Happy live'"),
//     };
    
//    // println!("new tweet by {}", my_tweet.summarize());  

//   ler s = String::from("hello world");
//   //notify(s);
//   notify(tweet_1);
//   notify(article_1);






// }








#[derive(Debug)]

struct Driver {
    driver_name:String,
    lisence:bool,
    driving_experience:String,
}

pub trait DriverType {
    fn car_driver (&self) -> String;
    fn truck_driver (&self) -> String;
}


impl DriverType for Driver {
    fn car_driver(&self) -> String{
        format!("Driver Name: {}\n Lisence: {}\n Driving Experience: {}",
        self.driver_name,
        self.lisence,
        self.driving_experience
        )
    }

    fn truck_driver(&self) -> String{
        format!("Driver Name: {}\n Lisence: {}\n Driving Experience: {}",
        self.driver_name,
        self.lisence,
        self.driving_experience
        )
    }

}


fn main () {
    let cr_1 = Driver {
        driver_name: "Afaque Ahmed".to_string(),
        lisence:true,
        driving_experience:"10 years".to_string()
    };

   let tr_1 =  Driver{
        driver_name: "Wahaj".to_string(),
        lisence:true,
        driving_experience:"15 years".to_string()
    } ;

    println!("{}",cr_1.car_driver()); 
    println!("{}",tr_1.truck_driver());
}
















