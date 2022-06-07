// https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // &s1 문법은 우리가 s1의 값을 참조하지만 소유하지는 않는 참조자를 생성하도록 해줍니다. 소유권을 갖고 있지는 않기 때문에,
    // 이 참조자가 가리키는 값은 참조자가 스코프 밖으로 벗어났을 때도 메모리가 반납되지 않을 것입니다.
    println!("The length of '{}' is {}.", s1, len);



    // let s = String::from("hello");
    // change(&s);

    println!("{}", get_string())
}

// 비슷한 이치로, 함수 시그니처도 &를 사용하여 인자 s의 타입이 참조자라는 것을 나타내고 있습니다. 설명을 위한 주석을 달아봅시다:
fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    s.len()
}
// 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기 때문에, 아무런 일도 발생하지 않습니다.

// 빌린 건 수정이 안 됨. mut을 붙이면 되긴 하는데 딱히?
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn get_string() -> String{
    String::from("hello")
}
