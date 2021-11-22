use rand::{Rng, thread_rng};

pub fn sort(nums: &mut Vec<i32>, l: usize, r: usize) {
    if l + 1 < r {
        let m = partition(nums, l, r);
        sort(nums, l, m);
        sort(nums, m + 1, r);
        // println!("{:?} with partition {}", nums, m);
    }
}

fn partition(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {

    // Get a random number
    let mut rng = thread_rng();
    let random_pos = rng.gen_range(l..r);

    nums.swap(r-1, random_pos);
    let mut low = l;
    for i in l..r-1 {
        if nums[i] < nums[r - 1] {
            nums.swap(i, low);
            low += 1;
        }
    }
    
    nums.swap(low, r - 1);
    low
}