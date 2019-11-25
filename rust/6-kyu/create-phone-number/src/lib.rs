/// # Create phone number
///
/// Kata Link:
/// https://www.codewars.com/kata/525f50e3b73515a6db000b83
///
/// Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
///
/// Example:
///
/// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
///
/// The returned format must be correct in order to complete this challenge.
/// Don't forget the space after the closing parentheses!
///
fn create_phone_number(numbers: &[u8]) -> String {

    if numbers.len() < 10 {
        panic!("The given array has too few numbers")
    }

    format!("({}{}{}) {}{}{}-{}{}{}{}",
            numbers[0],
            numbers[1],
            numbers[2],
            numbers[3],
            numbers[4],
            numbers[5],
            numbers[6],
            numbers[7],
            numbers[8],
            numbers[9]
    )
}

#[cfg(test)]
mod tests {
    use super::create_phone_number;

    #[test]
    fn returns_enumberspected() {
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
        assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
    }
}
