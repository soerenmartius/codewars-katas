/// # Complementary DNA
///
/// Deoxyribonucleic acid (DNA) is a chemical found in the nucleus of cells and carries the "instructions" for the development and functioning of living organisms.
/// If you want to know more http://en.wikipedia.org/wiki/DNA
///
/// In DNA strings, symbols "A" and "T" are complements of each other, as "C" and "G".
/// You have function with one side of the DNA (string, except for Haskell); you need to get the other complementary side.
/// DNA strand is never empty or there is no DNA at all (again, except for Haskell).
///
/// More similar exercise are found here http://rosalind.info/problems/list-view/ (source)
///
/// Kata Link:
/// https://www.codewars.com/kata/554e4a2f232cdd87d9000038
///
fn dna_strand(dna: &str) -> String {
    dna.chars().map(
        |x| match x {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => x
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_strand("AAAA"), "TTTT");
        assert_eq!(dna_strand("ATTGC"), "TAACG");
        assert_eq!(dna_strand("GTAT"), "CATA");
    }
}
