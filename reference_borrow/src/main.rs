fn main() {
    let s1 = String::from("hello");

    //borrow instance to calculate_length
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // let dange_instance = dangle();

    let no_dangle_instance = no_dangle();
}

fn calculate_length(s : &String) -> usize {
    s.len()
}

// fn change(some_string : &String) {
//     //referenced instance cannot change
//     some_string.push_str(", world");
// }

//inferenced instance expired when function end
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

