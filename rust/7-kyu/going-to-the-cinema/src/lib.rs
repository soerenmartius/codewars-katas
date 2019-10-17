/// # Going to the cinema
///
/// My friend John likes to go to the cinema. He can choose between system A and system B.
///
/// System A : buy a ticket (15 dollars) every time
/// System B : buy a card (500 dollars) and every time
///   buy a ticket the price of which is 0.90 times the price he paid for the previous one.
///
/// Kata Link:
/// https://www.codewars.com/kata/562f91ff6a8b77dfe900006e
///
/// ## Example: If John goes to the cinema 3 times:
///
/// System A : 15 * 3 = 45
/// System B : 500 + 15 * 0.90 + (15 * 0.90) * 0.90 + (15 * 0.90 * 0.90) * 0.90 ( = 536.5849999999999, no rounding for each ticket)
///
/// John wants to know how many times he must go to the cinema so that the final result of System B, when rounded up to the next dollar, will be cheaper than System A.
/// The function movie has 3 parameters: card (price of the card), ticket (normal price of a ticket), perc (fraction of what he paid for the previous ticket) and returns the first n such that
///
/// ceil(price of System B) < price of System A.
///
fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let mut a_total = 0;
    let mut b_total = card as f64;
    let mut times = 0;

    while a_total <= b_total.ceil() as i32 {
        let increment = ticket as f64 * perc.powi(times + 1);

        b_total += increment;
        a_total += ticket;
        times += 1;
    }

    times
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
        println!(" card: {:?};", card);
        println!("ticket: {:?};", ticket);
        println!("perc: {:?};", perc);
        let ans = movie(card, ticket, perc);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(500, 15, 0.9, 43);
        dotest(100, 10, 0.95, 24);
        dotest(0, 10, 0.95, 2);
    }
}
