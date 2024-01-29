impl Solution {
    pub fn minimum_sum(mut nums: Vec<i32>) -> i32 {
      if nums.len() < 3 { return -1; }
  
      let mut found_value: i32 = -1;
      let mut value_index_pairs: Vec<(i32, usize)> = nums.iter().copied().enumerate()
      .map(|(i, value)| (value, i))
      .filter(|&(_, i)| i != 0 && i != nums.len() - 1) //Ignore first and last elements
      .collect();
  
      value_index_pairs.sort_by(|a, b| a.0.cmp(&b.0));
  
      for &(_value, index) in &value_index_pairs {
          let min_left = *nums[..index].iter().min().unwrap();
          let min_right = *nums[index+1..].iter().min().unwrap();
  
          if nums[index] > min_left && nums[index] > min_right {
              let value = min_left + min_right + nums[index];
              if found_value == -1 || value < found_value {
                  found_value = value;
              }
          }
      }
  
      return found_value
  }
  }