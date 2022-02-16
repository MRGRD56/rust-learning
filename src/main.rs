fn main() {
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