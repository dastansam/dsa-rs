/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
fn two_sum(nums: Vec<u128>, target: u128) -> (usize, usize) {
    let mut complements = std::collections::HashMap::new();

    for entry in nums.iter().enumerate() {
        let complement = target - entry.1;

        if let Some(index) = complements.get(&complement) {
            return (entry.0, *index);
        }

        complements.insert(entry.1, entry.0);
    }

    // this is unreachable
    (0, 0)
}

#[test]
fn two_sum_works() {
    let nums = [0, 1, 2, 3, 4, 5, 6, 7];
    let ans = two_sum(nums.to_vec(), 9);
    assert!(ans == (5, 4));

    let nums = [2, 7, 11, 15].to_vec();
    assert!(two_sum(nums, 9) == (1, 0));
}
