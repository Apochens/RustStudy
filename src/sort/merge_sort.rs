pub fn sort(nums: &mut Vec<i32>, l: usize, r: usize) {
    // println!("-----Sorting from {} to {}----", l, r);
    if l < r - 1 {
        let m = (l + r) / 2;
        // println!("{}, {}, {}", l, m, r);
        sort(nums, l, m);
        sort(nums, m, r);
        merge(nums, l, m, r);
    }
    println!("{:?}", &nums);
}

fn merge(nums: &mut Vec<i32>, l: usize, m: usize, r: usize) {
    let temp = nums[l..m].to_vec();
    // println!("The nums from {} to {}: {:?}", l, m, temp);
    let mut p1 = 0;
    let mut p2 = m;
    let mut p = l;
    while p1 < temp.len() || p2 < r {
        if p1 == temp.len() {
            nums[p] = nums[p2]; p += 1; p2 += 1;
            continue;
        } 
        if p2 == r {
            nums[p] = temp[p1]; p += 1; p1 += 1;
            continue;
        }
        if temp[p1] < nums[p2] {
            nums[p] = temp[p1]; p += 1; p1 += 1;
        } else {
            nums[p] = nums[p2]; p += 1; p2 += 1;
        }
    }
}