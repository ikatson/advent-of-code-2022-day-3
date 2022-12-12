use std::io::Read;

use ad3p2::{s1, s2, s3};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("first arg should be filename");

    let mut f = std::fs::File::open(filename).expect("can't open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("can't read file");

    let s = s.as_bytes();

    println!("s1: {}", s1::process_buf(s));
    println!("s1 part 2: {}", s1::process_buf_part_2(s));
    println!("s2: {}", s2::process_buf(s));
    println!("s2 part 2: {}", s2::process_buf_part_2(s));
    println!("s3: {}", s3::process_buf(s));
    println!("s3 part2: {}", s3::process_buf_part_2(s));
}
