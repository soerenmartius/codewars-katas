///
/// # Convert a String to a Number!
///
/// We need a function that can transform a string into a number. What ways of achieving this do you know?
/// Note: Don't worry, all inputs will be strings, and every string is a perfectly valid representation of an integral number.
///
/// Kata Link:
/// https://www.codewars.com/kata/convert-a-string-to-a-number
///
/// ## Example
///
/// string_to_number("1234")  == 1234
/// string_to_number("605")   == 605
/// string_to_number("1405")  == 1405
/// string_to_number("-7")    == -7
///
fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::string_to_number;

    #[test]
    fn returns_expected() {
        assert_eq!(string_to_number("1234"), 1234);
        assert_eq!(string_to_number("605"), 605);
        assert_eq!(string_to_number("1405"), 1405);
        assert_eq!(string_to_number("-7"), -7);
    }
}
