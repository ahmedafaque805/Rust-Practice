pub mod student_register{
#[derive(Debug)]
pub struct Student {
    name:String,
    email:String,
    age:i8,
    single: bool,
}

impl Student {
  pub  fn new(name:String, email:String, age:i8, single:bool) ->Student
    {
        Student{
            name,
            email,
            age,
            single,
        }
    }

  pub  fn get_student(&self){
        println!("Name:{}\nEmail:{}\nAge:{}\nSingle:{}",self.name,self.email,self.age,self.single );
    }
}
}