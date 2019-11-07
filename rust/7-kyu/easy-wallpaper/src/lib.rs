/// # Easy Wallpaper
///
/// John wants to decorate a room with wallpaper. He's heard that making sure he has the right
/// amount of wallpaper is more complex than it sounds.
/// He wants a fool-proof method for getting it right.
/// John knows that the rectangular room has a length of l meters, a width of w meters, a height of
/// h meters. The standard width of the rolls he wants to buy is 52 centimeters.
/// The length of a roll is 10 meters. He bears in mind however, that it’s best to have an extra
/// length of wallpaper handy in case of mistakes or miscalculations so he wants to buy a length 15%
/// greater than the one he needs.
/// Last time he did these calculations he got a headache, so could you help John?
/// Your function wallpaper(l, w, h) should return as a plain English word in lower case the number
/// of rolls he must buy.
///
/// Kata Link:
/// https://www.codewars.com/kata/567501aec64b81e252000003
///
fn wall_paper(l: f64, w: f64, h: f64) -> String {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen", "twenty"
    ];

    let n = ((2.0 * w * h + 2.0 * h * l) * 1.15 / (0.52 * 10.0)).ceil() as usize;

    match l * h * w {
        0.0 => numbers.get(0).unwrap().to_string(),
        _ => numbers.get(n).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::wall_paper;

    fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
        println!("l: {:?}", l);
        println!("w: {:?}", w);
        println!("h: {:?}", h);
        let ans = wall_paper(l, w, h);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(6.3, 4.5, 3.29, "sixteen");
        dotest(6.3, 5.8, 3.13, "seventeen");
    }
}
