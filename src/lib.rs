struct MemchrSplit<'a> {
    buf: &'a [u8],
    needle: u8,
}

impl<'a> MemchrSplit<'a> {
    pub fn new(buf: &'a [u8], needle: u8) -> Self {
        Self { buf, needle }
    }
}

impl<'a> Iterator for MemchrSplit<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let offset = memchr::memchr(self.needle, self.buf)?;
        let ret = unsafe { self.buf.get_unchecked(..offset) };
        self.buf = &self.buf[offset + 1..];
        Some(ret)
    }
}

pub mod compartment {
    use std::hint::unreachable_unchecked;

    pub fn c1(c: &[u8]) -> u64 {
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

    // This one auto-vectorizes, but it's somehow detrimental at least on m1.
    pub fn c2(c: &[u8]) -> u64 {
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

    pub fn c3(c: &[u8]) -> u64 {
        let mut result = 0u64;
        for byte in c.iter().copied() {
            let bit = if byte >= b'a' {
                byte - b'a' + 1
            } else {
                byte - b'A' + 27
            };
            result |= 1 << bit;
        }
        result
    }
}

pub mod lsplit {
    use crate::MemchrSplit;

    pub fn l1(b: &[u8]) -> impl Iterator<Item = &[u8]> {
        b.split(|c| *c == b'\n').filter(|l| !l.is_empty())
    }

    pub fn l2(b: &[u8]) -> impl Iterator<Item = &[u8]> {
        MemchrSplit::new(b, b'\n')
    }
}

pub mod part1 {
    pub fn line(l: &[u8], compartment: impl Fn(&[u8]) -> u64) -> u32 {
        let half = l.len() / 2;
        let left = &l[..half];
        let right = &l[half..];

        let left_c = compartment(left);
        let right_c = compartment(right);

        let intersection = left_c & right_c;
        intersection.trailing_zeros()
    }

    pub fn process_buf_generic<'a, LINES, COMP>(lines: LINES, comp: COMP) -> u32
    where
        COMP: Fn(&[u8]) -> u64 + Copy,
        LINES: Iterator<Item = &'a [u8]>,
    {
        lines.map(|b| line(b, comp)).sum()
    }

    #[cfg(test)]
    mod tests {
        use crate::compartment;

        use super::line;

        #[test]
        fn t1_c1() {
            assert_eq!(line(b"fBDGBcBrGDvjPtPtPV", compartment::c1), 30);
        }

        #[test]
        fn t2_c1() {
            assert_eq!(
                line(b"ttgJtRGJQctTZtZT", compartment::c1),
                (b't' - b'a') as u32 + 1
            );
        }

        #[test]
        fn t1_c2() {
            assert_eq!(line(b"fBDGBcBrGDvjPtPtPV", compartment::c2), 30);
        }

        #[test]
        fn t2_c2() {
            assert_eq!(
                line(b"ttgJtRGJQctTZtZT", compartment::c2),
                (b't' - b'a') as u32 + 1
            );
        }

        #[test]
        fn t1_c3() {
            assert_eq!(line(b"fBDGBcBrGDvjPtPtPV", compartment::c3), 30);
        }

        #[test]
        fn t2_c3() {
            assert_eq!(
                line(b"ttgJtRGJQctTZtZT", compartment::c3),
                (b't' - b'a') as u32 + 1
            );
        }
    }
}

pub mod part2 {
    pub fn process_buf_generic<'a, LINES, COMP>(mut lines: LINES, comp: COMP) -> u32
    where
        COMP: Fn(&[u8]) -> u64,
        LINES: Iterator<Item = &'a [u8]>,
    {
        let mut result = 0u32;

        while let Some(el1) = lines.next() {
            let el2 = lines.next().unwrap();
            let el3 = lines.next().unwrap();

            let intersection = comp(el1) & comp(el2) & comp(el3);
            result += intersection.trailing_zeros();
        }
        result
    }
}

pub mod s1 {
    use crate::{compartment, lsplit, part1, part2};

    pub fn process_buf(b: &[u8]) -> u32 {
        part1::process_buf_generic(lsplit::l1(b), compartment::c1)
    }

    pub fn process_buf_part_2(b: &[u8]) -> u32 {
        part2::process_buf_generic(lsplit::l1(b), compartment::c1)
    }
}

pub mod s2 {
    use crate::{compartment, lsplit, part1, part2};

    pub fn process_buf(b: &[u8]) -> u32 {
        part1::process_buf_generic(lsplit::l1(b), compartment::c2)
    }

    pub fn process_buf_part_2(b: &[u8]) -> u32 {
        part2::process_buf_generic(lsplit::l1(b), compartment::c2)
    }
}

pub mod s3 {
    use crate::{compartment, part1, part2, MemchrSplit};

    pub fn process_buf(b: &[u8]) -> u32 {
        part1::process_buf_generic(MemchrSplit::new(b, b'\n'), compartment::c3)
    }

    pub fn process_buf_part_2(b: &[u8]) -> u32 {
        part2::process_buf_generic(MemchrSplit::new(b, b'\n'), compartment::c3)
    }
}
