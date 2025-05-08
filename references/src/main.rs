fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");
}
// "Borrow" s1 by using a reference to it in the function parameter
fn calculate_length(s: &String) -> usize {
  s.len()
  // s1 is not dropped because s is a reference to it -- not ownership.
}