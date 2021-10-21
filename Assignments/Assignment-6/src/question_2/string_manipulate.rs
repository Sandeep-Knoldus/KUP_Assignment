#[warn(dead_code)]
/// The function 'output' is used to give desired output and accepts three string as input
///
/// #Arguments
///
///str1 - first input string
///str2 - second input string
///str3 - third input string
///
/// #Return
///
/// Returns the String to give the desired output....
pub fn _output(str1: String, str2: String, str3: String) -> String {
    let char1: Vec<char> = str1.chars().collect();
    let char2: Vec<char> = str2.chars().collect();
    let char3: Vec<char> = str3.chars().collect();

    let length1 = str1.len();
    let length2 = str2.len();
    let length3 = str3.len();

    let mut str4: String = "".to_string();
    use std::cmp;
    if length1 == length2 && length2 == length3 {
        for loop1 in 0..length1 {
            if loop1 % 2 == 0 {
                let mut min1 = cmp::min(char1[loop1], char2[loop1]);
                min1 = cmp::min(min1, char3[loop1]);
                str4.push(min1);
            } else {
                let mut max1 = cmp::max(char1[loop1], char2[loop1]);
                max1 = cmp::max(max1, char3[loop1]);
                str4.push(max1);
            }
        }
    }
    str4
}
