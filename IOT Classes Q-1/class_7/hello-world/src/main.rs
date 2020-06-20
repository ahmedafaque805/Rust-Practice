//fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }
//}
// fn calculate_length(s: &String) -> usize {
//     s.len()
//  }

// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
//     change(&mut s);
//     println!("{}",s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }




// fn main()
// {
// let mut s = String::from("hello");

//     let r1 = &mut s;
//     println!("{}",r1);
//     let r2 = &mut s;
//     println!("{}",r2);
// }    

fn main(){
//     let v = vec! [1,2,3];
//     println!("{:?}",v)


//  v.push(30)
//  v.pop()


//   let mut v:Vec<(String,i32)>;
//   v = vec![("faheem".to_string(),45),         //0
//   ("ibad".to_string(),45),                    //1
//   ("afaque".to_string(),45)];                 //2
//   println!("{}",v[2].0);



// let x = vec![1,2,3];
// println!("{}",x.get(2).unwrap());


// let mut v = vec![100, 32, 57];
// for i in 0..v.len() {
// //     println!("{}", i+50);
//      v[i] += 50
//  }
// println!("{:?}",v)


// let mut v = vec![100,32,57];
// for i in &mut v {
//     *i += 50;
// }
// println!("{:?}",v);


// let hello = String::from("Olá");
// println!("{}",hello.len());



// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");
// let s = format!("{} - {} - {}", &s1,&s2,&s3);
// println!("{}", s);
// println!("{},{},{}",s1,s2,s3);

// let s1 = String::from("holag");
// for i in s1.chars()
//     {
//         println!("{}", s1)
//     }


// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);
// let change_value = scores.entry(String::from("blue")).or_insert(40);
//println!("{}", change_value);
//*change_value = 40;
// println!("{:?}",scores);




// use std::collections::HashMap;

// let text = "hello world wonderful world";

// let mut map = HashMap::new();

// for word in text.split_whitespace() {
//     println!("{}",map.entry(word).or_insert(0));
//     let count = map.entry(word).or_insert(0);
//     *count += 1;
// }

// println!("{:?}", map);







}









