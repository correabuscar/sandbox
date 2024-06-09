#![feature(const_mut_refs)] // https://github.com/rust-lang/rust/issues/57349
//concat strings and constant at compiletime, but call it at runtime on target array which can be on stack!
pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize> {
    _msg_len: usize,
}

impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    pub const fn get_message(buffer: &mut [u8]) -> usize {
        let mut len = 0;

        let part1 = "<invalid UTF-8 in this instance of ";
        let part2 = stringify!(NoAllocFixedLenMessageOfPreallocatedSize);
        let part3 = "::<";
        let part4 = ">>";

        len = copy_to_buf(buffer, len, part1.as_bytes());
        len = copy_to_buf(buffer, len, part2.as_bytes());
        len = copy_to_buf(buffer, len, part3.as_bytes());
        //len = copy_to_buf(buffer, len, &size_to_str::<SIZE>(SIZE));
        //len += size_to_str(SIZE, &mut buffer[len..]);
        len += size_to_str(SIZE, buffer, len);

        len = copy_to_buf(buffer, len, part4.as_bytes());

        len
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
    let mut i=index;
    while i<DIGITS_LEN {
    //for i in index..DIGITS_LEN {
        buffer[start+len] = digits[i];
        len += 1;
        i+=1;
    }
    len
}

//fn copy_to_buf(buf: &mut [u8], mut i: usize, bytes: &[u8]) -> usize {
//    buf[i..i + bytes.len()].copy_from_slice(bytes);
//    i += bytes.len();
//    i
//}
//
const fn copy_to_buf(buf: &mut [u8], i: usize, bytes: &[u8]) -> usize {
    let bytes_len = bytes.len();
    let mut j = 0;

    while j < bytes_len {
        buf[i + j] = bytes[j];
        j += 1;
    }

    i + bytes_len
}

fn main() {
    const MAX_LEN: usize = 4096; // Adjust this to your maximum expected message length
    let mut buffer = [0u8; MAX_LEN];

    let len = NoAllocFixedLenMessageOfPreallocatedSize::<44>::get_message(&mut buffer);
    let message_44 = std::str::from_utf8(&buffer[..len]).unwrap();

    println!("{}", message_44);
    assert_eq!(message_44, "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<44>>");

    //message_44 and thus buffer isn't used by the above anymore after this, so it's considered unborrowed

    let len = NoAllocFixedLenMessageOfPreallocatedSize::<4096>::get_message(&mut buffer);
    let message_4096 = std::str::from_utf8(&buffer[..len]).unwrap();
    println!("{}", message_4096);
    assert_eq!(message_4096, "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<4096>>");
}

