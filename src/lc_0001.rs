#[allow(dead_code)]
fn two_sum(num: &[i32], target: i32) -> Option<[usize; 2]> {
    let n = num.len();
    for i in 0..n {
        for j in i..n {
            if num[i] + num[j] == target {
                return Some([i, j]);
            }
        }
    }
    return None;
}

#[test]
fn two_sum_test() {
    let result = two_sum(&[2, 7, 11, 15], 9);
    assert_eq!(Some([0, 1]), result);
}
