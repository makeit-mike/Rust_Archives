/*
Source: LeetCode 3. Longest Substring Without Repeating Characters
Given a string s, find the length of the longest substring without repeating characters.

    ex1
        "abcabcbb" -> 3 (abc)
    ex2
        "bbbb" -> 1 (b)
    ex3
        "pwwkew" -> 3 (kew)
*/
use std::collections::HashMap;

fn main() {
    let l = length_of_longest_substring("abcabcbb".to_string());
    println!("abcabcbb - {}\r\n",l);

    let l = length_of_longest_substring("bbbb".to_string());
    println!("bbbb - {}\r\n",l);

    let l = length_of_longest_substring("pwwkew".to_string());
    println!("pwwkew - {}\r\n",l);

    let l = length_of_longest_substring("dvdf".to_string());
    println!("dvdf - {}\r\n",l);
}

// works for most scenarios, but is very brute force.
fn length_of_longest_substring_bruteforce(s: String) -> i32 {
    if s.len() <= 1 {
        return s.len() as i32;
    }

    let mut currMaxLen = 0;
    let mut maxLen = 0;
    let mut hash_map: HashMap<char, i32> = HashMap::new();

    for (i, item) in s.chars().enumerate() {
        if hash_map.contains_key(&item) {
            hash_map = HashMap::new();
            currMaxLen = 0;
        }
        currMaxLen = currMaxLen + 1;
        if currMaxLen >= maxLen {
            maxLen = currMaxLen;
        }
        hash_map.insert(item, i as i32);
    } 
    maxLen
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut hash_map = HashMap::new();
    let mut max_len = 0;
    let mut before = 0;
    let mut current = 0;
    for c in s.chars() {
        println!("current {}, before {}",current, before);
        if let Some(last) = hash_map.insert(c, current) {
            before = std::cmp::max(before, last);
        }
        max_len = std::cmp::max(max_len, current - before);
        current += 1;
    }
    println!();
    max_len
}