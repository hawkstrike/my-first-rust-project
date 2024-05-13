
pub fn multiple(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        // panic!("Cannot divide by zero");
    }
    
    a / b
}