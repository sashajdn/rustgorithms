use std::collections::HashSet;
use std::rc::Rc;

// checks if there are any duplicate numbers in the hashmap, if a duplicate is found,
// then we return true, otherwise return false using a hashmap to check for duplicates.
//
// O(T) -> O(n)
// O(S) -> O(n)
pub fn contains_duplicate_hs(nums: Vec<i64>) -> bool {
    let mut seen = HashSet::new();
    for &item in &nums {
        if !seen.insert(item) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_sanitycheck() {
        let r = Rc<0>;

        let result_empty = contains_duplicate_hs(vec![]);
        assert_eq!(result_empty, false);

        let result_no_dupe = contains_duplicate_hs(vec![1, 2, 3, 4]);
        assert_eq!(result_no_dupe, false);

        let result_dupe = contains_duplicate_hs(vec![1, 2, 3, 4, 2]);
        assert_eq!(result_dupe, true);
    }
}
