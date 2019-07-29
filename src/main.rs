fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("The new string is: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

