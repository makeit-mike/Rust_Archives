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

fn main() {
    let l = length_of_longest_substring("abcabcbb".to_string());
    println!("abcabcbb - {}\r\n",l);

    let l = length_of_longest_substring("bbbb".to_string());
    println!("bbbb - {}\r\n",l);

    let l = length_of_longest_substring("pwwkew".to_string());
    println!("pwwkew - {}\r\n",l);

    let l = length_of_longest_substring("dvdf".to_string());
    println!("dvdf - {}\r\n",l);

    let l = length_of_longest_substring("au".to_string());
    println!("au - {}\r\n",l);
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut p1 = -1;
    let mut max_length = 0;
    let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::with_capacity(128);

    for (p2, item) in s.chars().enumerate() {
        println!("{}", item);
        if let Some(p2) = map.insert(item, p2 as i32) {
           p1 = std::cmp::max(p1, p2);
        }
        max_length = std::cmp::max(max_length, (p2 as i32) - p1);
    } 
    max_length
}