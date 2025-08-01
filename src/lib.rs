use std::io;

pub fn takes_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap_or(0)
}
