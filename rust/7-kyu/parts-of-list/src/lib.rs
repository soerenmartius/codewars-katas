/// # Parts of List
///
/// Write a function partlist that gives all the ways to divide a list (an array) of at least two elements into two non-empty parts.
///
/// Kata Link:
/// https://www.codewars.com/kata/56f3a1e899b386da78000732
///
/// ## Example:
///
/// a = ["az", "toto", "picaro", "zone", "kiwi"]
/// --> [["az", "toto picaro zone kiwi"], ["az toto", "picaro zone kiwi"], ["az toto picaro", "zone kiwi"], ["az toto picaro zone", "kiwi"]]
///
fn part_list(arr: Vec<&str>) -> String {
    let mut res = String::new();
    for i in 1..arr.len() {
        res.push_str(format!("({}, {})", &arr[..i].join(" "), &arr[i..].join(" ")).as_str());
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
               "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        dotest(vec!["cdIw", "tzIy", "xDu", "rThG"],
               "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
    }
}
