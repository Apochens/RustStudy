pub fn sort(nums: Vec<i32>, ascending: bool) -> Vec<i32> {

    let mut nums = nums;
    if ascending == false {
        nums = nums.iter().map(|&x| -x).collect(); 
    }

    for i in 1..nums.len() {
        let key = nums[i];
        let mut j= (i - 1) as i32;
        while j >= 0 && nums[j as usize] > key {
            nums[j as usize + 1] = nums[j as usize];
            j -= 1;
        }
        nums[(j + 1) as usize] = key;
    }

    if ascending == false {
        nums.iter().map(|&x| -x).collect()
    } else {
        nums.to_owned()
    }
}