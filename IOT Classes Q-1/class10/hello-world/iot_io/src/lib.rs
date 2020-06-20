pub mod read_inputs{
    use std::io;
    pub fn read()-> String {
      let input = String::new();
      io::stdin().read_line(buf: &mut input);
      input
    }
}
