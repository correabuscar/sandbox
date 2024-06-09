#![feature(const_mut_refs)] // Enable mutable references in const functions
#![feature(generic_const_exprs)] // warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes

// so this works at compile time and returns a buffer+len but can't return an &str

pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize>;

impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    pub const fn get_err_message() -> ([u8; Self::err_msg_buffer_size()], usize) {
        let mut buffer = [0u8; Self::err_msg_buffer_size()];
        let mut len = 0;

        const PART1: &[u8] = b"<invalid UTF-8 in this instance of ";
        const PART2: &[u8] = stringify!(NoAllocFixedLenMessageOfPreallocatedSize).as_bytes();
        const PART3: &[u8] = b"::<";
        const PART4: &[u8] = b">>";

        len = copy_to_buf(&mut buffer, len, PART1);
        len = copy_to_buf(&mut buffer, len, PART2);
        len = copy_to_buf(&mut buffer, len, PART3);
        len += size_to_str(SIZE, &mut buffer, len);
        len = copy_to_buf(&mut buffer, len, PART4);

        (buffer, len)
    }

    pub const fn err_msg_buffer_size() -> usize {
        80+4 // Arbitrary size, should be enough for the err_message, if too low it fails compile time, so just increase it then, but 80+length of SIZE as &str is enough
    }
}

const fn size_to_str(size: usize, buffer: &mut [u8], start: usize) -> usize {
    const DIGITS_LEN: usize = 20; // Hoisted constant for maximum number of digits for a usize

    let mut digits = [b'0'; DIGITS_LEN];
    let mut num = size;
    let mut index = DIGITS_LEN;

    // Skip leading zeroes
    while num > 0 {
        index -= 1;
        digits[index] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    // If the number is zero, just return a single zero
    if index == DIGITS_LEN - 1 {
        buffer[start] = b'0';
        return 1;
    }

    // Copy the digits starting from the non-zero part into the buffer
    let mut len = 0;
    let mut i = index;
    while i < DIGITS_LEN {
        buffer[start + len] = digits[i];
        len += 1;
        i += 1;
    }
    len
}

const fn copy_to_buf(buf: &mut [u8], i: usize, bytes: &[u8]) -> usize {
    let bytes_len = bytes.len();
    let mut j = 0;

    while j < bytes_len {
        buf[i + j] = bytes[j]; // if out of bounds it means err_msg_buffer_size() is too low, like if u're using too big of a SIZE
        j += 1;
    }

    i + bytes_len
}

fn main() {
    const MSG_44: ([u8; NoAllocFixedLenMessageOfPreallocatedSize::<0>::err_msg_buffer_size()], usize) = NoAllocFixedLenMessageOfPreallocatedSize::<44>::get_err_message();
    let err_message_44 = std::str::from_utf8(&MSG_44.0[..MSG_44.1]).unwrap();
    println!("{}", err_message_44);
    assert_eq!(err_message_44, "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<44>>");

    const MSG_4096: ([u8; NoAllocFixedLenMessageOfPreallocatedSize::<0>::err_msg_buffer_size()], usize) = NoAllocFixedLenMessageOfPreallocatedSize::<4096>::get_err_message();
    let err_message_4096 = std::str::from_utf8(&MSG_4096.0[..MSG_4096.1]).unwrap();
    println!("{}", err_message_4096);
    assert_eq!(err_message_4096, "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<4096>>");
}

