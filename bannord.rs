use std::io;

fn main() -> io::Result<()> {
    let mut s = String::new();
    let mut m = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut s)?;
    stdin.read_line(&mut m)?;

    let result = m
        .split_whitespace()
        .map(|word| {
            if word.chars().any(|ch| s.contains(ch)) {
                "*".repeat(word.len())
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
    Ok(())
}
