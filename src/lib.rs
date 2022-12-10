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
            // Note lowercase always starts with 11, and uppercase always with 10.
            // This way we can distinguish them with bitwise ops.
            //
            // 'a'..'z' == 0b110_0001..0b111_1010
            // 'A'..'Z' == 0b100_0001..0b101_1010
            let byte = *byte;
            // This uses the fact that "a" is 1 if only the last 5 bits are taken.
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
    fn compartment(c: &[u8]) -> u64 {
        let mut result = 0u64;
        for byte in c.iter().copied() {
            let bit = match byte {
                b'a'..=b'z' => byte - b'a' + 1,
                _ => byte - b'A' + 27,
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

    pub fn process_buf(b: &[u8]) -> u32 {
        let mut result = 0u32;
        let mut prev = 0;
        for pos in memchr::memchr_iter(b'\n', b) {
            result += process_line(unsafe { b.get_unchecked(prev..pos) });
            prev = pos + 1
        }
        result
    }

    pub fn process_buf_part_2(b: &[u8]) -> u32 {
        let mut result = 0u32;
        let mut prev = 0;

        let mut it = memchr::memchr_iter(b'\n', b);

        while let Some(el1_end) = it.next() {
            let el2_end = it.next().unwrap();
            let el3_end = it.next().unwrap();

            let el1_s = unsafe { b.get_unchecked(prev..el1_end) };
            let el2_s = unsafe { b.get_unchecked(el1_end + 1..el2_end) };
            let el3_s = unsafe { b.get_unchecked(el2_end + 1..el3_end) };

            let intersection = compartment(el1_s) & compartment(el2_s) & compartment(el3_s);
            result += intersection.trailing_zeros();
            prev = el3_end + 1;
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
