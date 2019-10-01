///
/// # Highest and Lowest
///
/// In this little assignment you are given a string of space separated numbers, and have to return the highest and lowest number.
///
/// Kata Link:
/// https://www.codewars.com/kata/highest-and-lowest
///
/// ## Example
///
/// highAndLow("1 2 3 4 5");  // return "5 1"
/// highAndLow("1 2 -3 4 5"); // return "5 -3"
/// highAndLow("1 9 3 4 -5"); // return "9 -5"
///
/// ## Notes
///
/// - All numbers are valid Int32, no need to validate them.
/// - There will always be at least one number in the input string.
/// - Output string must be two numbers separated by a single space, and highest number is first.

fn high_and_low(numbers: &str) -> String {
    let vec: Vec<i32> = n.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    String::from(format!(
        "{:?} {:?}",
        vec.iter().max().unwrap(),
        vec.iter().min().unwrap()
    ))
}

#[test]
fn test_highest_and_lowest() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}
