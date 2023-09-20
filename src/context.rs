fn connection<F: FnOnce(i32) -> ()> (address: String, f: F ) {
    println!("Connection open to {}", address);
    f(123);
    println!("Connection closed");
}


fn main() {

  connection("target".to_string(), |_conn|{
      println!("Doing something well");
  });

  connection("target".to_string(), |_conn|{
      println!("Doing something wrong");
  });
  
  connection("target".to_string(), |_conn|{
      println!("Doing something miserably wrong");
      panic!("Boo"); // connection not closed
  });
  
}
