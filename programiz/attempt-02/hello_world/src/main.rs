fn main() {
    // generic function to find minimum between two inputs
    fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            return a;
        } else {
            return b;
        }
    }

    // call generic function with integer type as parameters    
    let result1 = min(2, 7);

    // call generic function with float type as parameters
    let result2 = min(2.1, 1.1);
    
    println!("Result1 = {}", result1);
    println!("Result2 = {}", result2);
}
