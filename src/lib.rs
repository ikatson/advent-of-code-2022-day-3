use std::hint::unreachable_unchecked;

pub fn compartment(c: &[u8]) -> u64 {
    let mut result = 0u64;
    for byte in c {
        let bit = match *byte {
            b'a'..=b'z' => *byte - b'a' + 1,
            b'A'..=b'Z' => *byte - b'A' + 27,
            _ => unsafe { unreachable_unchecked() },
        };
        result |= 1 << bit;
    }
    result
}

pub fn process_line(l: &[u8]) -> u32 {
    if l.is_empty() {
        return 0;
    }
    let half = l.len() / 2;
    let left = &l[..half];
    let right = &l[half..];

    let left_c = compartment(left);
    let right_c = compartment(right);

    let intersection = left_c & right_c;
    intersection.trailing_zeros()
}

pub fn process_buf(b: &[u8]) -> u32 {
    b.split(|c| *c == b'\n').map(process_line).sum()
}

#[cfg(test)]
mod tests {
    use super::process_line;

    #[test]
    fn t1() {
        assert_eq!(process_line(b"fBDGBcBrGDvjPtPtPV"), 30);
    }

    #[test]
    fn t2() {
        assert_eq!(process_line(b"ttgJtRGJQctTZtZT"), (b't' - b'a') as u32 + 1);
    }
}
