
/*
Condition: Find the maximum Achievable Number
Given two integers, num and t. A number is achievable if it can become equal to num after applying the following operation:
Increase or decrease the number by 1, and simultaneously increase or decrease num by 1.
Return the maximum achievable number after applying the operation at most t times.

Pseudocode:
    alongside t <i32> and num <i32>, there is one abstract result var, that is going to be increased 2 times ofr each t operation
*/


pub fn easy_2769 (num: i32, t: i32) {
    let mut ach_number: i32 = num;
    for _ in (0..t) {
        ach_number = ach_number + 2
    }
}