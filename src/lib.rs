pub mod s1 {
    use std::hint::unreachable_unchecked;

    fn compartment(c: &[u8]) -> u64 {
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
}

pub mod s2 {
    // This one auto-vectorizes, but it's somehow detrimental at least on m1.
    fn compartment(c: &[u8]) -> u64 {
        let mut result = 0u64;
        for byte in c {
            // 'a'..'z' == 0b110_0001..0b111_1010
            // 'A'..'Z' == 0b100_0001..0b101_1010
            let byte = *byte;
            let value = byte & 0b1_1111;
            // if uppercase, add 26 to the value
            // 0b11 - 0b11 == 0 (lowercase prefix)
            // 0b11 - 0b10 == 1 (uppercase prefix)
            let value = (0b11 - (byte >> 5)) * 26 + value;
            result |= 1 << value;
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
}

pub mod s3 {
    use std::hint::unreachable_unchecked;

    fn compartment(c: &[u8]) -> u64 {
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
        let half = l.len() / 2;
        let left = &l[..half];
        let right = &l[half..];

        let left_c = compartment(left);
        let right_c = compartment(right);

        let intersection = left_c & right_c;
        intersection.trailing_zeros()
    }

    pub fn process_buf(mut b: &[u8]) -> u32 {
        let mut result = 0u32;
        while !b.is_empty() {
            let end = memchr::memchr(b'\n', b).unwrap_or(b.len());
            result += process_line(&b[..end]);
            match b.get(end + 1..) {
                Some(p) => b = p,
                None => break,
            }
        }
        result
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
}
