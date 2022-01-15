fn main() {
    let arr = [
        "()",
        "(())",
        "(()()())",
        "((()))",
        ")(",
        "(()",
        "())",
        ")",
        "(",
        "",
        "(3*4)+(4x)",
    ];

    for x in arr.iter() {
        println!("{} is {}", x, valid_parenthesis(&x))
    }
}

pub fn valid_parenthesis(s: &str) -> bool {
    if s.len() == 0 { return false; }

    let mut leftPCount = 0;
    let mut rightPCount = 0;

    for c in s.chars() {
        if c == '(' { leftPCount += 1 }
        if c == ')' { rightPCount += 1 }

        if rightPCount > leftPCount { return false; }
    }

    return rightPCount == leftPCount;
}
