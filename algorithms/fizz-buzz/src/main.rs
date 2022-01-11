fn main() {
    print!("test");
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
     for n in 1..=n {
         fizzbuzz(n);
     }
}

fn fizzbuzz(n: u32) -> () { // -> () is optional
    if is_div_by(n, 15) {
        println!("fizzbuzz");
        return;
    }
    if is_div_by(n, 3) {
        println!("fizz");
        return;
    }
    if is_div_by(n, 5) {
        println!("buzz");
        return;
    } 
    println!("{}", n);
}

fn is_div_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0 // last line of a method that returns some datatype does not need "return" or a semicolon... weird
}