use crate::listnode::ListNode;

mod listnode;
struct Solution{}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let arr = s.as_bytes();
        let (mut l, mut ret) = (0_usize, 0_usize);
        let mut cnts =vec![0_i32; 128];
        for x in arr.iter().enumerate() {
            let idx = *x.1 as usize;
            cnts[idx] += 1;
            while cnts[idx] > 1 {
                cnts[arr[l] as usize ] -= 1;
                l += 1;
            }
            ret = ret.max((x.0 + 1 - l));
        }
        ret as i32
    }
}
impl Solution {
    pub fn find_mid(nums1: &Vec<i32>, nums2: &Vec<i32>, idx: usize) -> f64 {
        let (n1, n2 ) = (nums1.len(), nums2.len());
        let (mut l, mut r) = (0, 0);
        let mut left_idx = idx;
        loop {

            if l == n1 {
                return nums2[idx - 1- n1] as f64;
            }
            if r == n2 {
                return nums1[idx - 1 - n2] as f64;
            }
            if left_idx == 1 {
                return nums1[l].min(nums2[r]) as f64;
            }

            let half = left_idx / 2;
            let (new_idx1 , new_idx2) =  ((l + half).min(n1) - 1, (r + half).min(n2 ) - 1);
            if nums1[new_idx1] <= nums2[new_idx2] {
                left_idx -= new_idx1 - l + 1;
                l = new_idx1 + 1;
            } else {
                left_idx -= new_idx2 - r + 1;
                r = new_idx2 + 1;
            }
        }
        todo!()
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2 ) = (nums1.len(), nums2.len());
        let (idx1, idx2) = ((n1 + n2 + 1) / 2, (n1 + n2 + 1) / 2 + (n1 + n2 + 1) % 2);
        return (Self::find_mid(&nums1, &nums2, idx1) + Self::find_mid(&nums1, &nums2, idx2)) / 2_f64;
    }
}
#[test]
fn tesata(){
    let a = Solution{};
    let a = Solution::find_median_sorted_arrays(vec![1, 2], vec![3]);
    println!("{}", a);
}
#[cfg(test)]
mod test {
    #[test]
    fn testa() {

    }
}
