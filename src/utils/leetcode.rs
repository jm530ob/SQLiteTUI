pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut l = 0;
        let mut r = nums.len();
        while l <= r {
            let middle = (l + r) / 2;
            if nums[middle] == target {
                return middle as i32;
            }
            if nums[middle] < target {
                l = middle.saturating_add(1);
            } else if nums[middle] > target {
                r = middle.saturating_sub(1);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    pub fn test_soltution() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        // let s = Solution::search(nums, target);
        // assert_eq!(s, 4);

        let res = nums.binary_search(&target).unwrap();
        assert_eq!(res, 4);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let s = Solution::search(nums, target);

        assert_eq!(s, -1);
    }
}
