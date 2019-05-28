/*
 * Rustで定数。
 * CreatedAt: 2019-05-28
 */
fn main() {
    //const value: i32 = 100; // warning: constant `value` should have an upper case name
    //value = 200; // error[E0070]: invalid left-hand side expression
    //value: i32 = 200; // error[E0658]: type ascription is experimental (see issue #23416)
    //const value: i32 = 200; // error[E0428]: the name `value` is defined multiple times
    const VALUE: i32 = 100;
    println!("VALUE is {}", VALUE);
}

