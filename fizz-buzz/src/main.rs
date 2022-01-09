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
    
    if is_div_by(n, 3) {
        print!("fizz");
    }
    
    if is_div_by(n, 5) {
        print!("buzz");
    } 

    println!("{}", n);
}

fn is_div_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}