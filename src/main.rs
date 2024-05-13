use sub_mod_folder::sub_mod::print_hello;
use sub_mod_folder::sub_tree::sub_tree_fn::sub_tree_test;

mod sub_main;
mod sub_mod_folder;

fn main() {
    let a: i64 = 10;
    
    println!("a = {}", a);
    
    let b: i64 = a + 10;
    
    println!("sum = {}", sum(a, b));
    println!("a = {}", a);
    println!("b = {}", b);
    
    println!("Hello, world!");
    
    // let s1 = String::from("hello");
    let s1 = "hello";
    let s2 = s1;
    println!("{}, world!", s1);
    
    const TEMP: fn(i32, i32) -> i32 = sub_main::multiple;
    
    println!("multiple = {}", sub_main::multiple(10, 20));
    println!("multiple = {}", TEMP(10, 20));
    println!("divide = {}", sub_main::divide(10, 1));
    
    println!("----------------------------------");
    print_hello();
    sub_tree_test();
}

fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

/*fn sum(a: i64, b: i64) -> i64 {
    a + b
}*/