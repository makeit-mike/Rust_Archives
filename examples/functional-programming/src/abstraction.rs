
pub fn simple_function() {
    println!("All I do is print a line. I am pure.");
}

pub fn add(a: i32, b: i32) -> i32{
    a + b  // note the return type does not NEED "return ...;"
}

pub fn subtract(a: i32, b: i32) -> i32{
   return  a - b;  // but.. you are okay to add them if you want.
}

pub fn use_fn_as_var() {
    let fn_as_variable = add; // Yay, functions are First Class!
    println!("10 + 20 = {}", fn_as_variable(10,20)); 
}

pub fn higher_order_fn<F>(val: i32, s: F) -> i32 
    where F: Fn(i32) -> i32 {
    s(val)
}

pub fn add_one(x: i32) -> i32 {
    x+1
}

pub fn implement_higher_order_fn() {
    let use_higher_order_fn = higher_order_fn(20, add_one); 
    let use_anonymousfn_higher_order_fn = higher_order_fn(20, |x:i32| x + 1); // Same thing as the line above... just inlined the function (anon)
    println!("Add 1 to 20: {}", use_higher_order_fn);
    println!("Add 1 to 20.. anonymously.. {}", use_anonymousfn_higher_order_fn);
}

pub fn return_fn_from_fn<'a>(s:& 'a i32) -> Box< dyn Fn(i32) -> i32 + 'a > {
    Box::new(move |x:i32| x + s)
}

pub fn return_function_from_function() {
    let return_from_fn = return_fn_from_fn(&300);
    println!("val: {}", return_from_fn(60));
}
