fn main() {
   //string slice
   let s = String::from("hello world");
   // here starting index is 0 and ending index is equivalent to the length of that part of String
   let hello = &s[0..5];
   // This is similar to above code
   // let hello = &s[..5];

   let world = &s[6..11];

   println!("{}, {}", hello, world);

   let s1 = String::from("Grishmin");
   let len = s1.len();
   let slice_s1 = &s1[3..len];

   //This is similar to above code
   // let slice_s1 = &s1[3..];
   println!("{}", slice_s1);
   //to get the entire string
   let slice2 = &s1[..];
   println!("{}", slice2);
}
