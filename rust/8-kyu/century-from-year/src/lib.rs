///
/// # Century from Year
///
/// The first century spans from the year 1 up to and including the year 100, The second - from the year 101 up to and including the year 200, etc.
///
/// Kata Link:
/// https://www.codewars.com/kata/century-from-year4
///
/// ## Example
///
/// Given a year, return the century it is in.
/// Input , Output Examples ::
///
/// monkeyCount(10) // --> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// monkeyCount(1) // --> [1]
///
/// So the maximum value that you can obtain is 9.
///
fn century(year: u32) -> u32 {
    match year % 100 {
        0 => year / 100,
        _ => (year / 100) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::century;

    #[test]
    fn basic_tests() {
        assert_eq!(century(1900), 19);
        assert_eq!(century(1601), 17);
        assert_eq!(century(2000), 20);
        assert_eq!(century(89), 1);
    }
}
