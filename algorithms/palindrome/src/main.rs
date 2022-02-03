
fn is_palindrome_int(x: i64) -> bool {
    let y: String = x.to_string();
    y == reverse(&y)
}

fn is_palindrome(x: &String) -> bool {
    x.to_string() == reverse(x)
}


fn reverse(x: &String) -> String {
    x.chars().rev().collect::<String>()
}

fn main() {
    println!("Is {} a Palindrome? -> {}", 121, is_palindrome_int(121));
    println!("Is {} a Palindrome? -> {}", 123, is_palindrome_int(123));
    println!("Is abba a Palindrome? -> {}", is_palindrome(&"abba".to_string()));
    println!("Is hello a Palindrome? -> {}", is_palindrome(&"hello".to_string()));
}
