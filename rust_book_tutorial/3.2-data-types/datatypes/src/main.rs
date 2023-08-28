use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");    

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5];    
    let first = a[0];
    let second = a[1];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");    
}
