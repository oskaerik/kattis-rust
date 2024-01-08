use std::io;

fn main() -> io::Result<()> {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip)?;
    ip = ip.trim().to_owned();

    let result = if ip.contains(".") {
        ipv4(&ip)
    } else {
        ipv6(&ip)
    };

    println!("{}", result);
    Ok(())
}

fn ipv4(ip: &str) -> String {
    let substrings = ip.split('.').rev().collect::<Vec<_>>();
    substrings.join(".") + ".in-addr.arpa."
}

fn ipv6(ip: &str) -> String {
    let replaced = if ip.contains("::") {
        let colons_needed = 7 - ip.matches(':').count() + 2;
        ip.replace("::", &":".repeat(colons_needed))
    } else {
        ip.to_owned()
    };
    let full = replaced
        .split(":")
        .map(|s| format!("{:0>4}", s))
        .collect::<Vec<_>>()
        .join("");
    full.chars()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(".")
        + ".ip6.arpa."
}
