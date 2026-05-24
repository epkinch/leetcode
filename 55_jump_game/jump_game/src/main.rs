impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 { return true; }
        let mut furthest = 0;
        for (i, &num) in nums.iter().enumerate() {

            furthest = furthest.max(num as usize + i);
            if furthest >= nums.len() -1 { return true; }
            if furthest <= i { return false; }
        }
        false
    }
}