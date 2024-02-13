use std::fs;

fn main() {
    let dest_path = r"src\version.rs";
    fs::write(
        &dest_path,
        "pub static VERSION:u32 = 0;"
    ).unwrap();
}