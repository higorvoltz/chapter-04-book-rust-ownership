fn main() {
  {
    let _s = "hello"; //memory stack
    println!("inside scope variable _s is alive");
  }

  // println!("{}", _s);
  // |                  ^ not found in this scope
  let mut another = String::from("hello"); //memory heap
  another.push_str(", world");
  println!("{}", another);
  let _a = 5; // a is ownership of 5
  //let b = x; //error stack

  let s1 = String::from("oi");
  let s2 = s1; //occupy more memory

  println!("{}", s2);

  //println!("{}", s1); error value borrowed here after move
  //s1 is useless

  //Variable and Data Interacting with Clone

  let str1 = String::from("testing");
  let str2 = str1.clone(); //no broken

  println!("str1 is {str1} and str2 is {str2}");

  println!();

  let my_str = String::from("new test");
  takes_ownership(my_str);

  println!();



  let text = String::from("Rust");
  let len_text = calculate_length(&text); //reference of text variable

  println!("The length of '{}' is {}.", text, len_text);

  println!();

  let mut rust_str = String::from("Programming in Rust is...");
  change(&mut rust_str);

  let mut oop = String::from("learning OOP");
  {
    let r1 = &mut oop;
    println!("inside scope: {}", r1); // no problem
  }

  let r2 = &mut oop;
  println!("{}", r2); // no problem

  //dangling references

  let reference_to_nothing = dangle();


}

fn takes_ownership(some_string: String){
  println!("{}", some_string);
}

fn calculate_length(some_string: &String) -> usize {
  some_string.len()
}

fn change(some_string: &mut String){
  some_string.push_str(", awesome!");
}

fn dangle() -> String {
  let s = String::from("dangle");

  s
}




