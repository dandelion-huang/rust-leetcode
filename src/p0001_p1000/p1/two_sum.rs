use std::collections::HashMap;

// <HashTable>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement: i32 = target - num;

            if map.contains_key(&complement) {
                return vec![*map.get(&complement).unwrap() as i32, index as i32];
            }

            map.insert(num, index);
        }

        vec![]
    }
}
