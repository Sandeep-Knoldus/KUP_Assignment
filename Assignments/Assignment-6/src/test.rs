#[warn(unused_imports)]
#[cfg(test)]
#[test]
fn _generating_substring_test1() {
    use crate::question_1::generate::_generate_substring;
    assert_eq!(_generate_substring("pa".to_string()), ["p", "pa", "a"]);
}

#[test]
fn _search_substring_test1() {
    use crate::question_1::search::_search;
    assert_eq!(_search("Pankaj Chaudhury", "Cha"), "7".to_string());
}

#[test]
fn _max_min_test1() {
    use crate::question_2::string_manipulate::_output;
    assert_eq!(
        _output(
            "jjdhid".to_string(),
            "ikjhjk".to_string(),
            "rtysgi".to_string()
        ),
        "itdsgk".to_string()
    );
}
