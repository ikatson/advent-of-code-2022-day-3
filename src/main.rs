use std::io::Read;

use ad3p2::process_buf;

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .expect("first arg should be filename");

    let mut f = std::fs::File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let mut total = 0;

    for i in 0..1000 {
        total += process_buf(s.as_bytes());
    }
    println!("{total}");
    Ok(())
}
