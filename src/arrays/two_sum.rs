use std::collections::HashMap;

// Finds any two numbers that add up to the target and returns the indices of the two numbers
// as a vector.
//
// O(T) -> O(n)
// O(S) -> O(n)
#[inline]
pub fn two_sum(nums: &[i64], target: i64) -> Vec<usize> {
    if nums.len() < 2 {
        return vec![];
    }

    let mut prev_map: HashMap<i64, usize> = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        let diff: i64 = target - num;

        if let Some(&index) = prev_map.get(&diff) {
            return vec![index, i];
        }

        prev_map.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_sanitycheck() {
        let result_one = two_sum(&[2, 7, 11, 15], 9);
        assert_eq!(result_one, vec![0, 1]);

        let result_one = two_sum(&[2, 7, 11, 15, 3], 10);
        assert_eq!(result_one, vec![1, 4]);

        let result_empty = two_sum(&[2, 7, 11, 15, 3], 1);
        assert_eq!(result_empty, vec![]);
    }
}
