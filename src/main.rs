#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

use core::{ffi::CStr, fmt::Write};

use ad3p2::{s1, s2, s3};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        if writeln!(FdWriter::stderr(), "{msg}").is_ok() {}
    }
    unsafe { libc::exit(1) }
}

fn map_args(argc: i32, argv: *const *const u8) -> impl Iterator<Item = &'static CStr> {
    (0..argc).map(move |offset| unsafe { CStr::from_ptr(*argv.offset(offset as isize) as _) })
}

struct FdWriter {
    fd: i32,
}

impl FdWriter {
    const fn stdout() -> Self {
        Self { fd: 1 }
    }
    const fn stderr() -> Self {
        Self { fd: 2 }
    }
}

impl core::fmt::Write for FdWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { libc::write(self.fd, s.as_ptr() as _, s.len()) };
        Ok(())
    }
}

fn print_c_error_and_panic(m: &'static str) {
    write!(FdWriter::stderr(), "{m}: ").unwrap();
    unsafe { libc::perror(core::ptr::null()) };
    panic!("");
}

unsafe fn stat(fd: i32) -> libc::stat {
    let mut stat: core::mem::MaybeUninit<libc::stat> = core::mem::MaybeUninit::uninit();
    let res = libc::fstat(fd, stat.as_mut_ptr());
    if res == -1 {
        print_c_error_and_panic("can't stat file");
    }
    stat.assume_init()
}

unsafe fn mmap_file<'a>(name: &'static CStr) -> &'a [u8] {
    let fd = libc::open(name.as_ptr(), libc::O_RDONLY);
    if fd == -1 {
        print_c_error_and_panic("error opening file");
    }
    let stat = stat(fd);
    if stat.st_mode & libc::S_IFREG != libc::S_IFREG {
        panic!("not a regular file");
    }
    let size = stat.st_size;
    let mm = libc::mmap(
        core::ptr::null_mut(),
        size as usize,
        libc::PROT_READ,
        libc::MAP_PRIVATE,
        fd,
        0,
    );
    if mm.is_null() {
        print_c_error_and_panic("error calling mmap")
    }
    core::slice::from_raw_parts(mm as _, size as usize)
}

fn get_filename_from_args(argc: i32, argv: *const *const u8) -> &'static CStr {
    map_args(argc, argv)
        .nth(1)
        .expect("the first argument should be a string")
}

#[no_mangle]
pub extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
    let f = unsafe { mmap_file(get_filename_from_args(argc, argv)) };
    let result = s3::process_buf(f);
    writeln!(FdWriter::stdout(), "{result}").unwrap();

    // Similar to previous version, but unneccessary:
    unsafe { libc::exit(0) }
}

// fn main() {
//     let filename = std::env::args().nth(1).unwrap();

//     let mut f = std::fs::File::open(filename).unwrap();
//     let mut s = String::new();
//     f.read_to_string(&mut s).unwrap();

//     // println!("s1: {}", s1::process_buf(s.as_bytes()));
//     // println!("s1 part 2: {}", s1::process_buf_part_2(s.as_bytes()));
//     // println!("s2: {}", s2::process_buf(s.as_bytes()));
//     // println!("s2 part 2: {}", s2::process_buf_part_2(s.as_bytes()));
//     println!("s3: {}", s3::process_buf(s.as_bytes()));
//     // println!("s3 part2: {}", s3::process_buf_part_2(s.as_bytes()));
// }
