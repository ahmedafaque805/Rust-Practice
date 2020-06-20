fn main() {
  let avail = true;
 if avail == true {
   println!("Me yahan pey agaya hun!")
}
 else if avail == false{
   println!("me ghar pe hun!")
}
else {
   println!("me rastey me hn!")
 }



 let num = 0;
if num !=0{
println!("the number is not zero :{}", num)
}
let num = 24;
  
if num % 2 ==0{
println!("the number is even");
}
else {
println!("the number is odd");
}



let num_01 = 30;
let num_02 = 20;

if num_01<num_02{
 println!("the larger number is: {}",num_02)
}
else{
 println!("the larger number is: {}",num_01)
}



let _num = 24;
if _num % 12 == 0{
 println!("number is divided by: 12");
}
else if _num % 6 == 0{
 println!("number is divided by: 6")
}



let _numb = 2;
let _result = if _numb % 2 ==0{
 "even"
}
else{
 "odd"
};
println!("{}", _result);

 

let mut semester = 0; // variable/ loop variable initialization
while semester < 10{
 println!("the Semester is: {}", semester);
let mut course = 1;
while course <=6 {
 println!("the course is: {}.{}", semester, course);
course +=1;  // pulse or minus dono use ho sakte han
 }
semester +=1 ;
}


for num3 in 0..10{
    println!("the number is: {}", num3)
}


let _num4 = 10;
let _float_num = 20.2;
let _text = "hello data";

let mixdata = (_num4,_float_num,_text);
println!("{:?}", mixdata);
println!("text data: {}", mixdata.2);

let _sametypedata = [12,43,65,76,234];
println!("{:?}", _sametypedata);
println!("text data: {}", _sametypedata[2]);
}
