fn main() {
   //string slice
   let s = String::from("hello world");
// here starting index is 0 and ending index is equivalent to the length of that part of String
   let hello = &s[0..5];

   let world = &s[6..11];

   println!("{}, {}", hello, world);
}
