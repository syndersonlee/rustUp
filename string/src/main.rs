fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(&s4);
    println!("s4 is {}", s4);

    let mut s5 = String::from("lo");
    s5.push('l');
    println!("s5 is {}", s5);

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7;
    println!("s5 is {}", s8);

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s12 = s9 + "-" + &s10 + "-" + &s11;
    println!("s12 is {}", s12);

}
