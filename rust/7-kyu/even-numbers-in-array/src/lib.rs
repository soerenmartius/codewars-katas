/// # Even Numbers in Array
///
/// Given an array of digital numbers, return a new array of length number containing the last even numbers from the original array (in the same order).
/// The original array will be not empty and will contain at least "number" even numbers.
///
/// Kata Link:
/// https://www.codewars.com/kata/even-numbers-in-an-array
///
/// ## Example:
///
/// ([1, 2, 3, 4, 5, 6, 7, 8, 9], 3) => [4, 6, 8]
/// ([-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26], 2) => [-8, 26]
/// ([6, -25, 3, 7, 5, 5, 7, -3, 23], 1) => [6]
///
fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let even: Vec<_> = array.iter().cloned().filter(|&n| n % 2 == 0).collect();
    even[even.len() - number ..].to_vec()
}

// alternative solution
//fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
//    let mut s = array.iter()
//        .filter(|x| *x % 2 == 0)
//        .cloned()
//        .rev()
//        .take(number)
//        .collect::<Vec<i32>>();
//    s.reverse();
//    return s;
//}

#[test]
fn sample_tests() {
    assert_eq!(even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3), vec!(4, 6, 8));
    assert_eq!(even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2), vec!(-8, 26));
    assert_eq!(even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1), vec!(6));
}
