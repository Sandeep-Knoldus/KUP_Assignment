#[cfg(test)]
#[test]
pub fn generating_substring_test1() {
    use crate::question_1::generate::generate_substring;
    assert_eq!(generate_substring("pa".to_string()), ["p", "pa", "a"]);
}

#[test]
pub fn generating_substring_test2() {
    use crate::question_1::generate::generate_substring;
    assert_ne!(
        generate_substring("sit".to_string()),
        ["s", "i", "t", "si", "it"]
    );
}

#[test]
pub fn search_substring_test1() {
    use crate::question_1::search::search;
    assert_eq!(search("Pankaj Chaudhury", "Cha"), "7".to_string());
}

#[test]
pub fn search_substring_test2() {
    use crate::question_1::search::search;
    assert_ne!(search("Sandeep Chakraborty", "Cha"), "1".to_string());
}

#[test]
pub fn max_min_test1() {
    use crate::question_2::string_manipulate::output;
    assert_eq!(
        output(
            "jjdhid".to_string(),
            "ikjhjk".to_string(),
            "rtysgi".to_string()
        ),
        "itdsgk".to_string()
    );
}

#[test]
pub fn max_min_test2() {
    use crate::question_2::string_manipulate::output;
    assert_ne!(
        output(
            "jjdhid".to_string(),
            "ikjhjk".to_string(),
            "rtysgi".to_string()
        ),
        "jirdki".to_string()
    );
}
