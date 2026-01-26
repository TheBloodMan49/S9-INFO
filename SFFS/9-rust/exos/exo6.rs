#![allow(dead_code)]

fn main() {
    let x = 5;
    let y = x; // Valeur copiée
    println!("x = {}, y = {}", x, y);

    let point1 = Point { x: 10, y: 20 };
    let point2 = point1;
    println!("point1: {:?}, point2: {:?}", point1, point2);

    let data1 = Data {
        value: String::from("test"),
    };
    let data2 = data1.clone();
    println!("data1: {:?}, data2: {:?}", data1, data2);
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Data {
    value: String,
}
