use std::fmt::Binary;

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);


    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len(); <- error occurs

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;


    let s = 'z';
    let string = "zxc";


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);


    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);


    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


    another_function(5);
    binding();

    println!("{}", five());
    println!("{}", plus_one(five()));


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);


    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.iter().any(|&x| x == 2));
}


fn another_function(x: i32) {
    println!("Another function {}", x);
}

fn binding() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // 반환값에는 세미콜론 없애기
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
