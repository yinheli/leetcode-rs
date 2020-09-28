#[allow(dead_code)]
struct Solution {}

// https://leetcode.com/problems/two-sum/

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    let r = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(vec![0, 1], r);

    let r = Solution::two_sum(vec![3, 2, 4], 6);
    assert_eq!(vec![1, 2], r);

    let r = Solution::two_sum(vec![3, 3], 6);
    assert_eq!(vec![0, 1], r);
}
