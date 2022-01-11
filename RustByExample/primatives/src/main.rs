/*
    Scalar Types
    signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    floating point: f32, f64
    char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    bool either true or false
    and the unit type (), whose only possible value is an empty tuple: ()

    Compound Types
    arrays like [1, 2, 3]
    tuples like (1, true)
*/

fn main() {
    let some_outrageous_tuple = basic_primatives();
    let some_void_function = literals_and_operators();

}

fn basic_primatives() -> (bool, f64, i32, i32, bool) {
    let logical: bool = true && true;
    let a_float: f64 = 1.0;
    let an_int: i32 = 5;
    let another_int = 6i32; // sufficce notation.
    
    let mut inferred = 12; // initially assumes it is an i32.
    inferred = 4294967296i64;  // compiler realizes this shound be an i64.
    // inferred = true; // would error. types are not dynamic.
    let inferred = true; // sadly, variables can be completely overwritten with 'shadowing'.

    return (logical, a_float, an_int, another_int, inferred);
}

fn literals_and_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

fn arrays() {
    let fixed_size: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", fixed_size);

    let fixed_size_same_values = [0;500]; // this compiles to [i32; 500]
    println!("{:?}", fixed_size_same_values);

    let indexes_start_at_zero = fixed_size[0];
    
    let len_of_array = fixed_size.len();
    println!("Size of array: {}", len_of_array);
}


fn reverse_tuple(pair: (i32, bool)) -> (bool, i32) {
    // Kinda weird how you need to reassign the pair in order to access the children... maybe I am missing something.
    let (a,b) = pair;
    return (b,a);
}

fn reverse_tuple_without_variable(pair: (i32, bool)) -> (bool, i32) {
    // Oh... forgive my previous comment.. I guess this is how you would do that.
    return (pair.1, pair.0);
}

fn printing_tuples() {
    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
}