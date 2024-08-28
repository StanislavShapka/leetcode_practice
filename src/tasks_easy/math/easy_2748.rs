

pub fn easy_2748 (nums: Vec<i32>) -> i32 {
    if nums.len() == 0 || nums.len() == 1 {
        return 0
    }

    let mut i = 0;
    let mut j = 1;
    let mut result = 0;
    while i != nums.len() - 1 {

        if nums[i] > nums[j] {
            let r = nums[i].checked_div_euclid(nums[j]);
            println!("{:?}", r.unwrap())
        } else {
            let r = nums[j].checked_div_euclid(nums[i]);
            println!("{:?}", r.unwrap())
        }

        i = i + 1;
        j = j + 1;
    }
    result
}