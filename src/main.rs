use std::io::Read;

use ad3p2::{s1, s2, s3};

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .expect("first arg should be filename");

    let mut f = std::fs::File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    println!("s1: {}", s1::process_buf(s.as_bytes()));
    println!("s2: {}", s2::process_buf(s.as_bytes()));
    println!("s3: {}", s3::process_buf(s.as_bytes()));
    Ok(())
}
