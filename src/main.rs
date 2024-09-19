extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn subtract(a: i32, b: i32) -> i32;
}

fn main() {
    
    unsafe {
        let sum = add(5, 3);
        let diff = subtract(5, 3);
        println!("Sum: {}", sum);
        println!("Difference: {}", diff);
    }
}