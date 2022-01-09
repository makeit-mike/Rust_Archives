use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello world.\r\n");
    print_line("Hello, World!".to_string());
    ferris_say("Hello there".to_string());
}

fn print_line(message: String){
    println!("{}", message);
}

fn ferris_say(message: String){
    let stdout = stdout();
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}