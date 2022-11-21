fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    //상수 테스트
    const MAX_POINTS: u32 = 100000;

    //Shadowing 
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is : {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

}
