fn main() {
    let v1 : Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let v3 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v3[100];
    // let does_not_exist = v3.get(100);

    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    v4.push(6);
}
