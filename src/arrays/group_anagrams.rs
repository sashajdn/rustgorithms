use std::collections::HashMap;

// groups words as anagrams and returns them in unordered groups of anagrams.
//
// O(T) -> O(s * max(len(s))) where `s` is the number of strings passed, we assume that we only pass lowercase ASCII chars in the strings.
//                            and max(len(s)) is the length of the largest string passed.
// O(S) -> O(s) where `s` is the number of strings passed.
pub fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<String>> {
    let mut anagram_grouping: HashMap<String, Vec<String>> = HashMap::new();

    for str in strs {
        let encoded_key = encode_anagram(&str);

        anagram_grouping
            .entry(encoded_key)
            .or_default()
            .push(str.to_string());
    }

    anagram_grouping
        .into_iter()
        .map(|(_, group)| group)
        .collect()
}

fn encode_anagram(s: &str) -> String {
    let mut counts = [0; 26];

    for c in s.chars() {
        let index = (c as u8) - ('a' as u8);
        counts[index as usize] += 1;
    }

    counts
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| match value {
            0 => None,
            n => Some(format!("{}{}", index, n)),
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vector_structure(mut expected: Vec<Vec<String>>, mut got: Vec<Vec<String>>) {
        let normalize = |v: &mut Vec<Vec<String>>| {
            for inner in v.iter_mut() {
                inner.sort();
            }

            v.sort_by(|a, b| a.join("").cmp(&b.join("")));
        };

        normalize(&mut got);
        normalize(&mut expected);

        assert_eq!(expected, got);
    }

    #[test]
    fn group_anagrams_sanitycheck() {
        let result_one = group_anagrams(vec!["act", "pots", "tops", "cat", "stop", "hat"]);
        assert_vector_structure(
            result_one,
            vec![
                vec!["hat".to_string()],
                vec!["act".to_string(), "cat".to_string()],
                vec!["stop".to_string(), "pots".to_string(), "tops".to_string()],
            ],
        );
    }
}
