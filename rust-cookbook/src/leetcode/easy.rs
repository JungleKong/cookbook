
pub mod solution {
    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![map[&complement], i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}


#[cfg(test)]
mod test_leetcode {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
