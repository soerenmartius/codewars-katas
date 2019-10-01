///
/// # Camel Case
///
/// Consider an array of sheep where some sheep may be missing from their place. We need a function that counts the number of sheep present in the array (true means present).
///
/// ## Example
///
/// &[true,  true,  true,  false,
/// true,  true,  true,  true ,
/// true,  false, true,  false,
/// true,  false, false, true ,
/// true,  true,  true,  true ,
/// false, false, true,  true]
///
/// The correct answer would be 17.
///
/// Hint: Don't forget to check for bad values like null/undefined
fn camel_case(str: &str) -> String {
    let str = str.split_whitespace().map(|s| s.chars()[0].to_uppercase().to_string()).collect();

    return str;
}

#[cfg(test)]
mod tests {
    use crate::camel_case;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }
}
