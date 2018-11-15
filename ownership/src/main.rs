fn main() {
  /* Ownership & Validity */
  let s = String::from("🔥");

  // String has no Copy trait... s will be invalid after passing the first time
  print_string(s);
  // printString(s);

  // Atomics are copied by default, hence i stays valid after passing the first time
  let i: i16 = 42;
  print_int(i);
  print_int(i);


  /* References/Borrowing */
  let s = String::from("🎉"); // shadowed
  print_usize(give_length(&s));
  print_usize(give_length(&s)); // s still can be used because we only referenced it rather than taking ownership


  /* Slices: */
  let x = "manu"; // type is &str
  let s = String::from("Hello World");
  let word: &str = first_word(&s[..]);
  print_string_slice(&s[..]);
  print_string_slice(&s[..]);
  print_string_slice(word);
  // print_string(s); // fails because s is borrowed to word and therefor cannot be moved out
  // s.clear(); // fails word is assigned with a &str slice pointing to a part of s
}

fn print_string(s: String) {
  println!("{}", s);
}

fn print_string_slice(s: &str) {
  println!("{}", s);
}

fn print_int(i: i16) {
  println!("{}", i);
}

fn print_usize(u: usize) {
  println!("{}", u);
}

fn give_length(s: &String) -> usize {
  s.len()
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &c) in bytes.iter().enumerate() {
    if c == b' ' {
      return &s[0..i]; // &str slice from first to i-th character
    }
  }

  return &s[..]; // &str slice over the whole string
}
