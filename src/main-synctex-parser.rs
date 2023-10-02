extern crate libc;

use libc::{c_int, c_char};

#[link(name = "your_c_lib")]
extern {
    fn synctex_parser(
        input_file: *const c_char,
        line: c_int,
        column: c_int,
        page: *mut c_int,
        x: *mut c_int,
        y: *mut c_int,
    ) -> c_int;
}

fn main() {
    let input_file = "path/to/main.synctex.gz";
    let line = 10;
    let column = 5;
    
    let mut page: c_int = 0;
    let mut x: c_int = 0;
    let mut y: c_int = 0;

    let result = unsafe {
        synctex_parser(
            input_file.as_ptr() as *const c_char,
            line,
            column,
            &mut page,
            &mut x,
            &mut y,
        )
    };

    if result == 1 {
        println!("Page: {}", page);
        println!("X: {}", x);
        println!("Y: {}", y);
    } else {
        println!("Failed to parse SyncTeX data");
    }
}