use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use itertools::Itertools;

mod text_analyzer;
mod bytes;

fn main() -> std::io::Result<()> {
    let current_dir_ptr = &String::from(std::env::current_dir().unwrap().to_str().unwrap());

    println!("{}", current_dir_ptr);

    let file = File::open(Path::new(current_dir_ptr).join(r"src\resources\image.jpeg")).unwrap();
    let file_bytes = file.bytes().map(|x| x.unwrap()).collect_vec();
    let bytes_ptr = bytes::get_bytes_pointer(file_bytes);

    unsafe {
        let bytes_from_ptr = bytes::get_bytes(bytes_ptr);
        let mut result_file = File::create(Path::new(current_dir_ptr).join(r"src\resources\image_result.jpeg")).unwrap();
        return result_file.write_all(&bytes_from_ptr);
    }
}

fn text() {
    const BYTE_ARRAY: [u8; 3] = [0x01, 0x2d, 0xf0];

    for i in 0..BYTE_ARRAY.len() {
        let element = BYTE_ARRAY[i];
        match throw_if_one(element) {
            Ok(result) => println!("{}| Success: {:#02x} at {:p}", i, result, &result),
            Err(error) => println!("{}| Error: {}", i, error)
        }
    }

    let mut number = 54;
    increment(&mut number);
    println!("incremented: {}", number);
}

fn throw_if_one(arg: u8) -> Result<u8, String> {
    if arg == 1 {
        Err(format!("an error occurred with {} at {:p}", arg, &arg))
    } else {
        Ok(arg)
    }
}

fn increment(value: &mut i32) -> &mut i32 {
    *value = *value + 1;
    value
}