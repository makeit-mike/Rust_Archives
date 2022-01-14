fn main() {
    // let a = string_to_int_edgecases("words and 987".to_string());
    // println!("{}",a);

    // let a = string_to_int_edgecases("    -987".to_string());
    // println!("{}",a);

    let a = string_to_int_edgecases("123    oijsdf".to_string());
    println!("123    oijsdf -> {}",a);

    let a = string_to_int_edgecases("3.45".to_string());
    println!("3.45 -> {}",a);

    let a = string_to_int_edgecases("   +0 123".to_string());
    println!("   +0 123 ->{}",a);

    let a = string_to_int_edgecases("00000-42a1234".to_string());
    println!("00000-42a1234 -> {}",a);

    let a = string_to_int_edgecases(" ".to_string());
    println!("  -> {}",a);

    let a = string_to_int_edgecases("20000000000000000000".to_string());
    println!("20000000000000000000 -> {}",a);
    
    let a = string_to_int_edgecases("  0000000000012345678".to_string());
    println!("  0000000000012345678 -> {}",a);

    let a = string_to_int_edgecases("-13+8".to_string());
    println!("-13+8         -> {}",a);
}


pub fn string_to_int_simple(s: String) -> i32 {
    let mut s = s.replace(" ", "");
    let c = s.chars().nth(0).unwrap();
    if !c.is_digit(10) && c != '-' {
        return 0;
    }
    s.retain(|x| {['1', '2', '3', '4', '5', '6', '7', '8','9','0','-',].contains(&x)});
    s.parse().unwrap()
}



/*
Read in and ignore any leading whitespace.
Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
Return the integer as the final result.

Runtime: 0 ms, faster than 100.00% of Rust online submissions for String to Integer (atoi).
Memory Usage: 2 MB, less than 96.46% of Rust online submissions for String to Integer (atoi).

*/
pub fn string_to_int_edgecases(s: String) -> i32 {
    if s.is_empty() || s.len() == 0 { return 0; }

    let mut storage: String = "".to_string(); 
    let mut is_negative = false;
    let mut special_char_visited = false;

    for c in s.chars() {
        if c.is_digit(10) == false {
            if storage.len() == 0 {
                // while no digit has been processed, there are certain special chars that can be interpreted. (-,+, )
                if special_char_visited { return 0; }  // Only allowed 1 special char. Easy place to terminate.
                if c == ' ' { continue; }
                if c == '-' { is_negative = true; special_char_visited = true; continue; }
                if c == '+' { is_negative = false; special_char_visited = true; continue; }
                return 0;
            }
            else { break; } // Stop processing the second you hit a non valid digit.
        } 
        special_char_visited = true;
        if !(storage.len() == 0 && c == '0'){
            storage.push(c);
        }
    }

    if storage.len() == 0 { return 0; }
    if storage.len() > 10 {
        return if is_negative { std::i32::MIN } else { std::i32::MAX };
    }

    let mut a: i64;
    match storage.parse::<i64>() { Ok(n) => a = n,  Err(_n) => return 0, }  // If all else fails, terminate method with 0.

    if is_negative { a = a * -1; }
    if a > i32::MAX as i64 { return std::i32::MAX; }
    if a < std::i32::MIN as i64 { return std::i32::MIN; }
    a as i32
}
