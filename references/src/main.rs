fn main() {
  // Reference ex
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");
  // Mutable reference ex
  let mut s = String::from("hello");

  change(&mut s);
  
  // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
  // let mut s = String::from("hello");

  // let r1 = &mut s;
  // let r2 = &mut s;

  // println!("{}, {}", r1, r2);
  // Use curly brackets to create a new scope
  let mut s = String::from("hello");

  {
      let _r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let _r2 = &mut s;
  
  // 🆗 Last usage of the immutable references occurs before mutable reference is introduced
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{r1} and {r2}");
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{r3}");

    let reference_to_nothing = dangle();
}

// "Borrow" s1 by using a reference to it in the function parameter
fn calculate_length(s: &String) -> usize {
  s.len()
  // s1 is not dropped because s is a reference to it -- not ownership.
}

// Mutable reference function
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// Dangling Reference
fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
  } // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!