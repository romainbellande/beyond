pub mod controller;
pub mod hash;
pub mod web_error;

pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => panic!("zero is not a right argument to fibonacci()!"),
        1 => 1,
        2 => 2,
        /*
        50    => 12586269025,
        */
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
