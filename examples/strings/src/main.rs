// Primarily there are two types of strings in Rust.

pub fn types_of_strings() {
    let _type1: String = String::from("I am a type 1 string");
    let _type2: &str = "I am a type 2 string";
}

pub fn when_to_use_type1() {
    // Use when you need full String capabilities, and are okay with heap allocation
    return;
}

pub fn when_to_use_type2() {
    // When you do not need to modify the string.
    return;
}

pub fn convert_to_owned_string(s: &str) -> String {
    s.to_string()
}

pub fn convert_to_string_slice() -> &str {
    let s: String = String::from("I am some string");
    &s
}

pub fn memory(){
    let s = " hello "; // basically s is just a reference to " hello " on the heap.
    /*
        Stack: [ ptr *, len 7]
        Heap:  [..., " ", h, e, l, l, o, " "]
    */
    let t = s.trim();
     /*
        // in normal languages.
        Stack: [ ptr *, len 5]
        Heap:  [..., " ", h, e, l, l, o, " ", h, e, l, l, o]

        // in rust
        Stack: [ ptr * + 1, len 5]
        Heap:  [..., " ", h, e, l, l, o, " "]
                        [^^^^^^^^^^^^^^]

        Pretty sure this only works as long as your variable is immutable.
    */
}

pub fn string_literals() {
    let str1 = "hello"; // Known at compile time.
    let str2 = "hello".to_string(); // Not known at compile time, this will need to be allocated on the heap to return val
}


fn main() {

    println!("Hello, world!");
}
