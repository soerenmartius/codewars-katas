///
/// # Even or Odd
///
/// Create a function (or write a script in Shell) that takes an integer as an argument and returns "Even" for even numbers or "Odd" for odd numbers.
///
/// Kata Link:
/// https://www.codewars.com/kata/even-or-odd
///
fn even_or_odd(i: i32) -> &'static str {
    match i % 2 {
        0 => "Even",
        _ => "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::even_or_odd;

    #[test]
    fn test_even_or_odd() {
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
        assert_eq!(even_or_odd(7), "Odd");
        assert_eq!(even_or_odd(-1), "Odd");
    }
}
