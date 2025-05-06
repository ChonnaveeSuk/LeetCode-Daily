struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        
        // Iterate through the array
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if num_map.contains_key(&complement) {
                return vec![num_map[&complement], i as i32];
            }
            num_map.insert(num, i as i32);
        }

        vec![]
    }
}


fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    println!("Input: nums = {:?}, target = {}", nums1, target1);
    println!("Output: {:?}", Solution::two_sum(nums1, target1)); 
    
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    println!("Input: nums = {:?}, target = {}", nums2, target2);
    println!("Output: {:?}", Solution::two_sum(nums2, target2)); 
    
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("Input: nums = {:?}, target = {}", nums3, target3);
    println!("Output: {:?}", Solution::two_sum(nums3, target3)); 
}