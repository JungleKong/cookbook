use std::collections::HashMap;


mod Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                result.push(*map.get(&complement).unwrap());
                result.push(i as i32);
                return result;
            }
            map.insert(num, i as i32);
        }
        result
    }
}
