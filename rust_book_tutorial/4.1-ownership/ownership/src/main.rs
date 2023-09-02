fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2); // it works because s1 is moved to s2
                                // println!("{}, world!", s1);
                                //     error[E0382]: borrow of moved value: `s1`
                                //  --> src/main.rs:5:28
                                //   |
                                // 2 |     let s1 = String::from("hello");
                                //   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
                                // 3 |     let s2 = s1;
                                //   |              -- value moved here
                                // 4 |
                                // 5 |     println!("{}, world!", s1);
                                //   |                            ^^ value borrowed here after move
                                //   |
                                //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
                                // help: consider cloning the value if the performance cost is acceptable
                                //   |
                                // 3 |     let s2 = s1.clone();
                                //   |                ++++++++

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
