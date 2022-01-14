fn main() {
    let a = string_to_int("31234".to_string());
    println!("{}",a);
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