impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create hashmap
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, current) in nums.iter().enumerate() {
        //let cached: Option<&i32> = map.get(&current);

        match map.get(current) {
            Some(value) => {
                
                return vec![*value, i as i32];
            }
            None => {
                map.insert(target - current, i as i32);
            }
        }
    }
    
    vec![]
    
    }
}