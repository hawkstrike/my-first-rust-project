
fn main() {
    let a: i64 = 10;
    
    println!("a = {}", a);
    
    let b: i64 = a + 10;
    
    println!("sum = {}", sum(a, b));
    println!("a = {}", a);
    println!("b = {}", b);
    
    println!("Hello, world!");
    
    let s1 = String::from("hello");
    // let s1 = "hello";
    let s2 = s1;
    println!("{}, world!", s1);
}

fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

/*fn sum(a: i64, b: i64) -> i64 {
    a + b
}*/