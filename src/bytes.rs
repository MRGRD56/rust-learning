pub struct BytesPointer {
    first_byte_ptr: *const u8,
    offset: usize
}

pub fn get_bytes_pointer(bytes: Vec<u8>) -> BytesPointer {
    BytesPointer {
        first_byte_ptr: bytes.as_ptr(),
        offset: bytes.len()
    }
}

pub unsafe fn get_bytes(bytes_ptr: BytesPointer) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut ptr: *const u8 = bytes_ptr.first_byte_ptr;
    while ptr > bytes_ptr.first_byte_ptr.add(bytes_ptr.offset) {
        bytes.push(*ptr);
        ptr = ptr.add(1);
    }

    bytes
}