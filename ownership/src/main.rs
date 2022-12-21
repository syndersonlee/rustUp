fn main() {
    // let s = String::from("hello");
    // let s1 = String::from("hello");
    // let s2 = s1;
    // //s1 is invalid
    // println!("{}", s2);

    //소유권
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;                      
    makes_copy(x);
}

fn takes_ownership (some_string : String) {
    println!("{}", some_string);
}

fn makes_copy (some_integer : i32) {
    println!("{}", some_integer);
}
 