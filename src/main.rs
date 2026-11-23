fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("args: {}", args.len());
}

#[allow(dead_code)]
fn trim_newlines(s: &str) -> &str { s.trim_end_matches(['\r', '\n']) }

