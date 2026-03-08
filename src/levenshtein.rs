use anyhow::{Context, Result};

pub fn get_levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    if s1_len == 0 {
        return s2_len;
    }

    if s2_len == 0 {
        return s1_len;
    }

    let mut cache: Vec<usize> = (1..).take(s1_len).collect();
    let mut result = 0;

    for (s2_index, s2_code) in s2.chars().enumerate() {
        result = s2_index;
        let mut s1_distance = s2_index;

        for (s1_index, s1_code) in s1.chars().enumerate() {
            let s2_distance = if s1_code == s2_code {
                s1_distance
            } else {
                s1_distance + 1
            };

            s1_distance = cache[s1_index];

            result = if s1_distance > result {
                if s2_distance > result {
                    result + 1
                } else {
                    s2_distance
                }
            } else if s2_distance > s1_distance {
                s1_distance + 1
            } else {
                s2_distance
            };

            cache[s1_index] = result;
        }
    }
    result
}

pub fn get_closest_match(input: &str, list: &[String]) -> Result<Option<String>> {
    const DISTANCE_THRESHOLD: usize = 3;

    let mut results = list
        .iter()
        .filter_map(|e| {
            let distance = get_levenshtein_distance(input, e);
            if distance < DISTANCE_THRESHOLD {
                Some((e, distance))
            } else {
                None
            }
        })
        .collect::<Vec<(&String, usize)>>();
    
    results.sort_by(|a, b| a.1.cmp(&b.1));

    if results.is_empty() {
        return Ok(None)
    }

    Ok(Some(results.first().expect("no command matches").0.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

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
}
