// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }
// // 여기서 &을 없애면 소유권을 calculate_length에 넘겨주기 때문에 s1을 사용할 수 없다.
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.


// ======================================================================


// // it doesn't work because s is borrowed as immutable
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


// ======================================================================


// // it works
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// ======================================================================


// it doesn't work because s is borrowed as immutable
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }
// error[E0499]: cannot borrow `s` as mutable more than once at a time
//  --> src/main.rs:5:14
//   |
// 4 |     let r1 = &mut s;
//   |              ------ first mutable borrow occurs here
// 5 |     let r2 = &mut s;
//   |              ^^^^^^ second mutable borrow occurs here
// 6 |
// 7 |     println!("{}, {}", r1, r2);
//   |                        -- first borrow later used here

// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `ownership` due to previous error


// ======================================================================


// but it works
// fn main() {
//     let mut s = String::from("hello");    
//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;

//     println!("{}", r2);




//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);

// }


// // ======================================================================


// fn main() {
//     let mut s = String::from("hello");    
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }


// fn main() {
//     let t = ([1; 2], [3; 4]);
//     let (a, b) = t;
//     println!("{}", a[0] + t.1[0]); 
//   }




// fn main() {
//   let s = String::from("hello");
//   let s2;
//   let b = true;
//   if b {
//     println!("b is true");
//     s2 = s;  // 여길 거치지 않아도 소유권이 이전됨 why?
//   }
//   println!("{}", s);
// }


// 이 형태면 실행 됨
fn main() {
  let s = String::from("hello");
  // let s2;
  let b = true;
  if b {
    println!("b is true");
    // s2 = s;  // 여길 거치지 않아도 소유권이 이전됨 why?
  }
  println!("{}", s);
  println!("{}", s);  
}


// 아래 코드에서 역참조가 일어나는 횟수: 3번
// y의 실질적인 자료형: <&Box<i32>>
// fn main() {
//   let x = Box::new(0);
//   let y = Box::new(&x);
// }