use crate::levenshtein::get_levenshtein_distance;

#[test]
fn levenshtein_distance_1() {
    let result = get_levenshtein_distance("kitten", "sitting");
    let expected = 3;
    assert_eq!(result, expected);
}

#[test]
fn levenshtein_distance_2() {
    let result = get_levenshtein_distance("uninformed", "uniformed");
    let expected = 1;
    assert_eq!(result, expected);
}

#[test]
fn levenshtein_distance_3() {
    let result = get_levenshtein_distance("--", "--help");
    let expected = 4;
    assert_eq!(result, expected);
}

