pub fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let zcount = nums1.iter().filter(|x| **x == 0).count();
    if zcount != nums2.len() {
        panic!("Invalid input! nums1 has {} '0' elements, but {} are required", zcount, nums2.len());
    }

    let mut merged: Vec<i32> = Vec::new();
    merged.extend_from_slice(
            nums1.into_iter().filter(|x| *x != 0).collect::<Vec<i32>>().as_slice());
    merged.extend_from_slice(nums2.as_slice());
    merged.sort();
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(merge(vec![1,2,3,0,0,0], vec![2,5,6]), vec![1,2,2,3,5,6]);
        assert_eq!(merge(vec![1,-1,1,9,0,0], vec![2,5]), vec![-1,1,1,2,5,9]);
    }

    #[test]
    #[should_panic]
    fn test_insufficient_uninitialized_values() {
        merge(vec![1,2,3,4,0], vec![5,6]);
    }
}
