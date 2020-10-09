fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    mutable_reference();
}

fn calculate_length(s: &String) -> usize {
    s.len()

    
}

//we have  to use &mut to mutate reference value
fn mutable_reference(){
    let mut s1 = String::from("Grishmin");

    change(&mut s1);

    println!("{}", s1);
}

fn change(s:&mut String){
    s.push_str(" karki");
}
