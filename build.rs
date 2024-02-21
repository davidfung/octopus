use std::fs;

fn main() {
    const PATH: &str = r"src/version.rs";
    let buf = fs::read_to_string(PATH).unwrap();
    let s = buf.trim_end_matches(";");
    let v:Vec<&str> = s.split_terminator("=").collect();
    let i:i32 = v[1].trim().parse().unwrap();
    let content =  format!("pub static VERSION:u32 = {};", i+1);
    fs::write(PATH, content).unwrap();
}

#[test]
fn test_build_script() {
    println!("Test");
}