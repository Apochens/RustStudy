use core::num;

#[derive(Debug)]
pub struct Node {
    sum: i32,
    left_pos: usize,
    right_pos: usize
}

pub fn max_subarray(array: Vec<i32>) -> Option<Node> {

    if array.len() == 0 {
        return None;
    }

    let mut max_subarrays: Vec<Node> = Vec::new();
    let init = Node {sum: array[0], left_pos: 0, right_pos: 1};
    max_subarrays.push(init);

    for i in 1..array.len() {
        if max_subarrays[i-1].sum < 0 {
            max_subarrays.push(Node{sum: array[i], left_pos: i, right_pos: i+1});
        } else {
            max_subarrays.push(Node{sum: max_subarrays[i-1].sum + array[i], 
                                    left_pos: max_subarrays[i-1].left_pos, 
                                    right_pos: i+1});
        }
    }

    let mut pos = 0;
    for i in 1..max_subarrays.len() {
        if max_subarrays[i].sum > max_subarrays[pos].sum {
            pos = i;
        }
    }

    Some(Node{sum: max_subarrays[pos].sum, 
                left_pos: max_subarrays[pos].left_pos,
                right_pos: max_subarrays[pos].right_pos})
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return ;
    }

    let mut pos = (n + m) as usize;
    let mut pos1 = m as usize;
    let mut pos2 = n as usize;

    while pos != 0 {
        if pos1 == 0 {
            nums1[pos - 1] = nums2[pos2 - 1];
            pos -= 1;
            pos2 -= 1;
            continue;
        }
        if pos2 == 0 {
            nums1[pos - 1] = nums1[pos1 - 1];
            pos -= 1;
            pos1 -= 1;
            continue;
        }
        if nums1[pos1-1] >  nums2[pos2-1] {
            nums1[pos - 1] = nums1[pos1 - 1];
            pos1 -= 1;
        } else {
            nums1[pos - 1] = nums2[pos2 - 1];
            pos2 -= 1;
        }
        pos -= 1;
    }
}

// 最长单调递增子序列 O(n^2) 动态规划
pub fn longest_subarray_n2(nums: Vec<i32>) -> Vec<i32> {
    let mut pred = vec![0usize; nums.len()];
    let mut count = vec![0usize; nums.len()];

    let mut max_pos = 0usize;
    for i in 0..nums.len() {
        count[i] = 1usize;
        pred[i] = i;
        for j in 0..i {
            if nums[i] > nums[j] && count[j] + 1 > count[i] {
                pred[i] = j;
                count[i] = count[j] + 1;
                if count[i] > count[max_pos] { max_pos = i; }
            }
        }
    }

    let mut sub_array: Vec<i32> = Vec::new();
    while pred[max_pos] != max_pos {
        sub_array.push(nums[max_pos]);
        max_pos = pred[max_pos];
    }
    sub_array.push(nums[max_pos]);
    sub_array.reverse();

    sub_array
}

// 最长单调递增子序列 O(nlgn) 贪心 + 二分查找
pub fn longest_subarray_nlgn(nums: Vec<i32>) -> Vec<i32> {
    let mut pred: Vec<usize> = vec![0; nums.len()];
    let mut min_pos: Vec<usize> = vec![0];
    let mut min: Vec<i32> = vec![nums[0]];
    let mut len = 0usize;

    for (i, &num) in nums.iter().enumerate() {
        pred[i] = i;
        if num > min[len] {
            pred[i] = min_pos[len];
            min_pos.push(i);
            min.push(num);
            len += 1;
        } else {
            match min.binary_search(&num) {
                Ok(pos) | Err(pos)=> {
                    if pos != 0 {
                        pred[i] = min_pos[pos - 1];
                    }
                    min[pos] = num;
                    min_pos[pos] = i;
                }
            }
        }
    }

    len = min_pos[len];
    let mut ret = vec![];
    while pred[len] != len { ret.push(nums[len]); len = pred[len]; }
    ret.push(nums[len]);

    ret.reverse();
    ret
}

// 矩阵链乘法 动态规划
pub fn matrix_chain_order(array: Vec<i32>) {
    
    fn print_optimal_parens(order: &Vec<Vec<usize>>, i: usize, j: usize) {
        if i == j { print!("A{}", i); }
        else {
            print!("(");
            print_optimal_parens(order, i, order[i][j]);
            print_optimal_parens(order, order[i][j]+1, j);
            print!(")");
        }
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![0; array.len()]; array.len()];
    let mut order: Vec<Vec<usize>> = vec![vec![0; array.len()]; array.len()];

    for k in 2..array.len() {
        for i in 1..=array.len()-k {
            dp[i][i+k-1] = i32::MAX;
            for j in i..i+k-1 {
                let temp = dp[i][j] + dp[j+1][i+k-1] + array[i-1]*array[j]*array[i+k-1];
                if temp < dp[i][i+k-1] {
                    dp[i][i+k-1] = temp;
                    order[i][i+k-1] = j;
                }
            }
        }
    }

    print_optimal_parens(&order, 1, array.len()-1);
}

// 钢条切割 动态规划
pub fn cut_rod(values: Vec<i32>, len: usize, c: i32) -> i32 {

    let mut dp: Vec<i32> = vec![0; len+1];
    let mut cut_pos: Vec<usize> = vec![0; len+1];

    for i in 1..=len {
        let mut q = i32::MIN;
        for j in 1..=i {
            let new_value = dp[i-j] + values[j] - c;
            if q < new_value {
                q = new_value;
                cut_pos[i] = j;
            }
        }
        dp[i] = q;
    } 
    println!("{:?}", &cut_pos);
    
    let mut last_cut_pos = len;
    while last_cut_pos > 0 {
        print!("{} ", cut_pos[last_cut_pos]);
        last_cut_pos = last_cut_pos - cut_pos[last_cut_pos];
    }
    println!();

    dp[len]
}

pub fn n_th_fib(n: i32) -> i32 {
    match n {
        0 => { 0 },
        1 => { 1 },
        _ => {
            let mut x = 0;
            let mut y = 1;
            let mut n_th_fib = 0;
            for _ in 2..=n {
                n_th_fib = x + y;
                x = y;
                y = n_th_fib;
            }
            n_th_fib
        }
    }
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp_max: Vec<i32> = vec![1; len];
    let mut dp_min: Vec<i32> = vec![1; len];
    dp_max[0] = nums[0];
    dp_min[0] = nums[0];
    
    let mut max = i32::MIN;
    for i in 1..len {
        if nums[i] < 0 {
            dp_max[i] = if dp_min[i-1] < 0 { nums[i] * dp_min[i-1] } 
                        else { nums[i].max(nums[i] * dp_max[i-1]) };
            dp_min[i] = if dp_max[i-1] > 0 { nums[i] * dp_max[i-1] }
                        else { nums[i].min(nums[i] * dp_min[i-1]) };
        } else {
            dp_max[i] = nums[i].max(nums[i] * dp_max[i-1]);
            dp_min[i] = nums[i].min(nums[i] * dp_min[i-1]);
        }
            
        max = max.max(dp_max[i]);
    }

    println!("{:?}", dp_max);
    println!("{:?}", dp_min);

    max
}