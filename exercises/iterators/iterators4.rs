// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // *** answer 1 ***
    match num {
        n if n == 0 => 1,
        n if n > 1 => num * factorial(n - 1),
        _ => num
    }
    // For an extra challenge, don't use:
    // - recursion
    // *** anwser 2 ***
    // [..=(num as i64)].iter().fold(num, |acc,n| acc * (num - 1))
    // Execute `rustlings hint iterators4` for hints.

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
