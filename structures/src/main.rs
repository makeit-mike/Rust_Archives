/*
    Types of structs:
        Tuple Structs (named tuples)
        C based Structs
        Unit Structs (fieldless/generics)
*/

fn main() {
    println!("Hello, world!");
}

struct UnitStruct;

#[derive(Debug)] // Allows printing out struct using println("{:?}")
struct NamedStruct {
    x: String,
    y: f32
}

struct TupleStruct(i32, String);

fn inline_struct() {
    let a = NamedStruct { x: "Test".to_string(), y: 5.0 };
    println!("{:?}", a);
}

