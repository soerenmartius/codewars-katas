/// # Descending Order
///
/// Your task is to make a function that can take any non-negative integer as a argument
/// and return it with its digits in descending order. Essentially, rearrange the digits
/// to create the highest possible number.
///
/// Kata Link:
/// https://www.codewars.com/kata/5467e4d82edf8bbf40000155
///
/// ## Examples
/// Input: 21445 Output: 54421
/// Input: 145263 Output: 654321
/// Input: 123456789 Output: 987654321
///
fn descending_order(x: u64) -> u64 {

    use std::iter::FromIterator;

    let mut r = x.to_string()
        .chars()
        .collect::<Vec<char>>();

    r.sort_by(|a, b| b.cmp(a));

    String::from_iter(r)
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::descending_order;

    #[test]
    fn returns_expected() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
