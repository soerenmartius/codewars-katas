///
/// # Expression Matter
///
/// Given three integers a ,b ,c, return the largest number obtained after inserting the following operators and brackets: +, *, ().
///
/// Kata Link:
/// https://www.codewars.com/kata/expressions-matter
///
/// ## Example
///
/// With the numbers are 1, 2 and 3 , here are some ways of placing signs and brackets:
///
/// 1 * (2 + 3) = 5
/// 1 * 2 * 3 = 6
/// 1 + 2 * 3 = 7
/// (1 + 2) * 3 = 9
///
/// So the maximum value that you can obtain is 9.
///
/// ## Notes
///
/// - The numbers are always positive.
/// - The numbers are in the range (1  ≤  a, b, c  ≤  10).
/// - You can use the same operation more than once.
/// - It's not necessary to place all the signs and brackets.
/// - Repetition in numbers may occur .
/// - You cannot swap the operands. For instance, in the given example you cannot get expression (1 + 3) * 2 = 8.
///
fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a + b + c, a * b * c, a * (b + c), c * (a + b)].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::expressions_matter;

    #[test]
    fn test_expression_matter() {
        assert_eq!(expressions_matter(2, 1, 2), 6);
        assert_eq!(expressions_matter(1, 1, 1), 3);
        assert_eq!(expressions_matter(2, 1, 1), 4);
        assert_eq!(expressions_matter(1, 2, 3), 9);
        assert_eq!(expressions_matter(1, 3, 1), 5);
        assert_eq!(expressions_matter(2, 2, 2), 8);

        assert_eq!(expressions_matter(5, 1, 3), 20);
        assert_eq!(expressions_matter(3, 5, 7), 105);
        assert_eq!(expressions_matter(5, 6, 1), 35);
        assert_eq!(expressions_matter(1, 6, 1), 8);
        assert_eq!(expressions_matter(2, 6, 1), 14);
        assert_eq!(expressions_matter(6, 7, 1), 48);

        assert_eq!(expressions_matter(2, 10, 3), 60);
        assert_eq!(expressions_matter(1, 8, 3), 27);
        assert_eq!(expressions_matter(9, 7, 2), 126);
        assert_eq!(expressions_matter(1, 1, 10), 20);
        assert_eq!(expressions_matter(9, 1, 1), 18);
        assert_eq!(expressions_matter(10, 5, 6), 300);
        assert_eq!(expressions_matter(1, 10, 1), 12);
    }
}
