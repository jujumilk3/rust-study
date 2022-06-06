// https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.

    let s1 = String::from("hello");
    // let s2 = s1; // double free 오류 발생 가능성으로 인해 컴파일오류 발생
    // 이걸 때리는 순간 s1이 참조하고 있던 메모리 주소는 무효화됨. 소유권이 s2로 이동했기 때문.
    // 러스트는 절대 deep copy를 하지 않음.

    println!("{}, world!", s1);


    let s1 = String::from("hello");
    let s2 = s1.clone(); // 복사가 필요하다면 이렇게.

    println!("s1 = {}, s2 = {}", s1, s2);


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // 이건 String과는 다르게 정수형과 같이 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장되기 때문에,
    // 실제 값의 복사본이 빠르게 만들어질 수 있음. 이는 변수 y가 생성된 후에 x가 더 이상 유효하지 않도록 해야할 이유가 없다는 뜻
    // 바꿔 말하면, 여기서는 깊은 복사와 얕은 복사 간의 차이가 없다는 것으로,
    // clone을 호출하는 것이 보통의 얕은 복사와 아무런 차이점이 없어 우리는 이를 그냥 버릴 수 있다는 것


    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);            // s의 값이 함수 안으로 이동했습니다...
    // println!("{}", s);                        // ... 그리고 이제 더이상 유효하지 않습니다.
    // String은 Copy trait이 없는 타입이고 s의 값이 함수 한으로 이동(moved)됐고
    // 함수 안에서 drop되어 해제됐기 때문에 밖에서 다시 참조할 수가 없는 거임!


    let x = 5;                           // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                // x가 함수 안으로 이동했습니다만,
                                              // i32는 Copy가 되므로, x를 이후에 계속 사용해도 됩니다.





    let s1 = gives_ownership();            // gives_ownership은 반환값을 s1에게
                                                  // 이동시킵니다.
    println!("{}", s1);

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2);            // s2는 takes_and_gives_back 안으로
                                                  // 이동되었고, 이 함수가 반환값을 s3으로도
                                                  // 이동시켰습니다.
    println!("{}", s3);
    // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
    // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로 벗어나서 drop이 호출됩니다.
}


fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                             // 호출한 쪽으로 이동시킵니다.
    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                      // 안으로 들어왔습니다.
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
