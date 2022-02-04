fn average_without_overflow(a: i32, b: i32) -> i32 {
    /* Let's calculate average of two integers,
    * 
    *                       avg = floor((a + b) / 2)
    *
    * The usual way to calculate average of two integers 
    * is to use  integer division. 
    *          
    * But this can lead to overflow.
    * For example, if a = i32::MAX,  b = 1, then in rust 
    * comiler will get panicked.
    * 
    * Here we use the following trick to avoid overflow.
    *                   avg  = (a & b) + ((a ^ b) >> 1)
    * Explanation:
    * 
    */

    return (a & b) + ((a ^ b) >> 1);
}

fn average_with_overflow(a: i32, b: i32) -> i32 {
    let avg = (a+b)/2;
    return avg;
}

fn main() {
    let a = std::i32::MAX;
    let b = 1i32;
    println!("a={}, b={}", a, b);
    let c = average_without_overflow(a, b);
    println!("Average without overflow: {}", c);

    let c = average_with_overflow(a, b);
    println!("Average with overflow: {}", c);
}
