//! Common utility methods for Platform Editor implementations.

/// Returns the number of digits of a `usize`.
pub fn digit_count(int: usize) -> usize {
    if int == 0 {
        return 1;
    }
    let mut number = int;
    let mut digits = 0;
    while number > 0 {
        number /= 10;
        digits += 1;
    }
    digits
}
