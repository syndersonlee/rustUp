/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/
struct Point<T, U> {
    x : T,
    y : U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other : Point<V, W>) -> Point<T, W> {
        Point {
            x : self.x,
            y : other.y,
        }
    }
}

struct Point2<T> {
    x : T,
    y : T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    /*
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
    */
    let integer = Point { x : 5, y : 10 };
    let float = Point { x : 1.0, y : 4.0};
    

    let p = Point2 { x : 5, y : 10};
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
