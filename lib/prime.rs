#[no_mangle]
pub fn isPrime(num: i32) -> bool {
    if num < 2 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    for i in (3..=(num as f64).sqrt() as i32).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }

    true
}