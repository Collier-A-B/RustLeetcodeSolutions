use std::collections::HashMap;

pub struct Solution {

}

impl Solution{
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1,-1];

        let mut summation_compliment_map: HashMap<i32, i32> = HashMap::new();

        for indx in 0..nums.len() {
            if summation_compliment_map.contains_key(&nums[indx]) {
                let first_indx: i32 = summation_compliment_map.get(&nums[indx]).map_or(0, |&v| v);
                result[0] = first_indx;
                result[1] = indx.try_into().expect("");
                break;
            }

            summation_compliment_map.insert(target - nums[indx], indx.try_into().expect(""));
        }

        return result;
    }
}