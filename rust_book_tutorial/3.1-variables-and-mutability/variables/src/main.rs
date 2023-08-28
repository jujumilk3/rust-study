fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");    // error


    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");    // error

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



    // shadow
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // 여기에서는 str이었다가 let을 통한 재정의로 int가 되었지만

    let mut spaces = "   ";
    spaces = spaces.len();  // error
    // 이것은 mut이지만 type이 바뀌는 것은 아니기 때문에 error가 난다.
    //|     spaces = spaces.len();  // error
    //|              ^^^^^^^^^^^^ expected `&str`, found `usize`
}
