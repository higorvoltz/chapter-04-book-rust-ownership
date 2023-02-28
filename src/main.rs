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
}