///
/// # Count the Monkeys
///
/// You take your son to the forest to see the monkeys. You know that there are a certain number there (n), but your son is too young to just appreciate the full number, he has to start counting them from 1.
/// As a good parent, you will sit and count with him. Given the number (n), populate an array with all numbers up to and including that number, but excluding zero.
///
/// Kata Link:
/// https://www.codewars.com/kata/count-the-monkeys
///
/// ## Example
///
/// With the numbers are 1, 2 and 3 , here are some ways of placing signs and brackets:
/// ```
/// monkeyCount(10) // --> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// monkeyCount(1) // --> [1]
/// ```
/// So the maximum value that you can obtain is 9.
///
fn monkey_count(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

#[cfg(test)]
mod tests {
    use super::monkey_count;

    #[test]
    fn test_monkey_count() {
        assert_eq!(monkey_count(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(monkey_count(1), vec![1]);
        assert_eq!(monkey_count(0), vec![]);
        assert_eq!(monkey_count(12), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}
