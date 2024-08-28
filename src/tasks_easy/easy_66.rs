
// Given two binary strings a and b, return their sum as a binary string.

// Example 1:
// Input: a = "11", b = "1"
// Output: "100"

// Example 2:
// Input: a = "1010", b = "1011"
// Output: "10101"
pub fn easy_66 () {

    let _ = local(String::from("11"), String::from("1"));
    let _ = local(String::from("1010"), String::from("1011"));
}
fn local (a: String, b: String) -> String {
    // iteration over bytes (reversed) and ->
    let mut s = a.bytes().rev().map(|b| Some(b - b'0')).chain(std::iter::repeat(None))
        .zip(b.bytes().rev().map(|b| Some(b - b'0')).chain(std::iter::repeat(None))).scan(0, |carry,(a, b)| {
        if a.is_some() || b.is_some() || *carry == 1 {
            let sum = a.unwrap_or(0) + b.unwrap_or(0) + *carry;
            let rez = ((sum & 1) + b'0') as char;
            *carry = sum >> 1;
            Some(rez)
        } else {
            None
        }
    }).collect::<String>();
    s.chars().rev().collect()
}

//Step 1 - Plan
/*
Binary sum is:

0 + 0 = 0
1 + 0 = 1
1 + 1 = 0 (1 Rest)

1. we can iterate over both Strings and check the condition. Some between states can be stored into a counter
2. we can convert them all into u8 and make a lot of conditions for add.
*/