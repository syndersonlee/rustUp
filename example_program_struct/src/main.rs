struct Rectangle {
    length: u32,
    width: u32,
}


fn main() {
    let rect1 = Rectangle {
        length : 50,
        width : 30
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectange : &Rectangle) -> u32 {
    rectange.length * rectange.width
}
