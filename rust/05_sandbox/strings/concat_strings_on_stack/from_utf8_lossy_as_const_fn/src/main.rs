#![feature(effects)]
#![feature(const_mut_refs)] // Enable mutable references in const functions

//rust 1.81.0 nightly tested here, won't work on 1.76.0
//this is: from_utf8_lossy() on stack and as const fn, needs a destination, so won't return &str, but buffer+len as struct type

pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize>;

impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    pub const fn get_message() -> Message<100> {
        const PART1: &[u8] = b"<invalid UTF-8 in this instance of ";
        const PART2: &[u8] = stringify!(NoAllocFixedLenMessageOfPreallocatedSize).as_bytes();
        const PART3: &[u8] = b"::<";
        const PART4: &[u8] = b">>";

        let mut buffer = [0u8; 100]; // Arbitrary size, should be enough for the message
        let mut len = 0;

        len = copy_to_buf(&mut buffer, len, PART1);
        len = copy_to_buf(&mut buffer, len, PART2);
        len = copy_to_buf(&mut buffer, len, PART3);
        len += size_to_str(SIZE, &mut buffer, len);
        len = copy_to_buf(&mut buffer, len, PART4);

        Message { buffer, len }
    }

    const fn validate_utf8(input: &[u8]) -> Result<usize, (usize, Option<usize>)> {
        let mut i = 0;
        while i < input.len() {
            let byte = input[i];
            if byte < 128 {
                i += 1;
                continue;
            }

            let valid_up_to = i;

            if byte >= 192 && byte < 224 && i + 1 < input.len() && input[i + 1] & 192 == 128 {
                i += 2;
            } else if byte >= 224
                && byte < 240
                && i + 2 < input.len()
                && input[i + 1] & 192 == 128
                && input[i + 2] & 192 == 128
            {
                i += 3;
            } else if byte >= 240
                && byte < 248
                && i + 3 < input.len()
                && input[i + 1] & 192 == 128
                && input[i + 2] & 192 == 128
                && input[i + 3] & 192 == 128
            {
                i += 4;
            } else {
                return Err((valid_up_to, Some(1)));
            }
        }

        Ok(i)
    }

    pub const fn from_utf8_lossy(input: &[u8]) -> Message<1024> {
        const REPLACEMENT: &[u8] = b"\xEF\xBF\xBD"; // UTF-8 for replacement character U+FFFD
        let mut buffer = [0u8; 1024];
        let mut len = 0;

        let mut i = 0;
        while i < input.len() && len < buffer.len() {
            //let result = std::str::from_utf8(&input[i..]);//doneTODO: use this? should be same but maybe better?
            let result = Self::validate_utf8(&input[i..]); // XXX: in 1.76.0 rust, this line won't compile: "E0015: cannot call non-const operator in constant functions calls in constant functions are limited to constant functions, tuple structs and tuple variants", but in nightly on playground it does! 1.81.0
            match result {
                Ok(valid_up_to) => {
                    if len + valid_up_to > buffer.len() {
                        break;
                    }
                    let mut j = 0;
                    while j < valid_up_to {
                        buffer[len] = input[i + j];
                        len += 1;
                        j += 1;
                    }
                    i += valid_up_to;
                }
                Err((valid_up_to, error_len)) => {
                    let invalid_sequence_length = match error_len {
                        Some(len) => len,
                        None => 1,
                    };

                    let mut j = 0;
                    while j < valid_up_to {
                        buffer[len] = input[i + j];
                        len += 1;
                        j += 1;
                    }//while

                    let mut k = 0;
                    while k < REPLACEMENT.len() {
                        if len < buffer.len() {
                            buffer[len] = REPLACEMENT[k];
                            len += 1;
                        } else {
                            break;
                        }
                        k += 1;
                    }//while

                    i += valid_up_to + invalid_sequence_length;
                }
            }//match
        }//while

        Message { buffer, len }
    }

    pub const fn from_utf8_lossy_2(input: &[u8]) -> Message<1024> {
        const REPLACEMENT: &[u8] = b"\xEF\xBF\xBD"; // UTF-8 for replacement character U+FFFD
        let mut buffer = [0u8; 1024];
        let mut len = 0;

        let mut i = 0;
        while i < input.len() && len < buffer.len() {
            let result = std::str::from_utf8(&input[i..]);//doneTODO: use this? should be same but maybe better?
            match result {
               Ok(valid) => {
                    let valid_bytes = valid.as_bytes();
                    if len + valid_bytes.len() > buffer.len() {
                        break;
                    }
                    let mut j = 0;
                    while j < valid_bytes.len() {
                        buffer[len] = valid_bytes[j];
                        len += 1;
                        j += 1;
                    }
                    break;
                }
               Err(e) => {
                    let valid_up_to = e.valid_up_to();
                    let invalid_sequence_length = match e.error_len() {
                        Some(len) => len,
                        None => 1,
                    };

                    let mut j = 0;
                    while j < valid_up_to {
                        buffer[len] = input[i + j];
                        len += 1;
                        j += 1;
                    }//while

                    let mut k = 0;
                    while k < REPLACEMENT.len() {
                        if len < buffer.len() {
                            buffer[len] = REPLACEMENT[k];
                            len += 1;
                        } else {
                            break;
                        }
                        k += 1;
                    }//while

                    i += valid_up_to + invalid_sequence_length;
                }
            }//match
        }//while

        Message { buffer, len }
    }
}

pub struct Message<const BUFFER_SIZE: usize> {
    buffer: [u8; BUFFER_SIZE],
    len: usize,
}

impl<const BUFFER_SIZE: usize> std::ops::Deref for Message<BUFFER_SIZE> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: We assume that the buffer contains valid UTF-8 data up to `self.len`.
        unsafe { std::str::from_utf8_unchecked(&self.buffer[..self.len]) }
    }
}

// that buffer in arg there, needs: #![feature(const_mut_refs)] // Enable mutable references in const functions
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
        buf[i + j] = bytes[j]; // if out of bounds it means err_msg_max_buffer_size() is too low, like if u're using too big of a SIZE
        j += 1;
    }

    i + bytes_len
}

fn main() {
    const MSG_44: Message<100> = NoAllocFixedLenMessageOfPreallocatedSize::<44>::get_message();
    let message_44: &str = &MSG_44;
    println!("{}", message_44);
    assert_eq!(
        message_44,
        "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<44>>"
    );

    let invalid_utf8 = b"Valid\xE2\x9D\xA4 text \x80\x90Invalid \xE2\x9D\xA4UTF-8";
    let msg = NoAllocFixedLenMessageOfPreallocatedSize::<0>::from_utf8_lossy(invalid_utf8);
    let msg_2 = NoAllocFixedLenMessageOfPreallocatedSize::<0>::from_utf8_lossy_2(invalid_utf8);
    let msg_str: &str = &msg;
    let msg_str_2: &str = &msg_2;
    println!("{}", msg_str);
    println!("{}", msg_str_2);
    assert_eq!(msg_str, "Valid❤ text ��Invalid ❤UTF-8");
    assert_eq!(msg_str_2, "Valid❤ text ��Invalid ❤UTF-8");
    assert_eq!(msg_str_2, msg_str);
}
