

fn main() {
    println!("This is how user input works: \r\nType some response. . .\r\n");
    let user_input = read_line();
    println!("You typed: {}", user_input);
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Couldn't read line");    
    input
}