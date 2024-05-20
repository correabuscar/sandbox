fn overwrite_msb_u64_with_i32(u64_value: u64, i32_value: i32) -> u64 {
    // Shift the i32 value to the left by 32 bits to align it with the MSBs of the u64
    let u64_value_to_insert = i32_value as u64;
    let shifted_u64_value_to_insert = u64_value_to_insert << 32;

    // Clear the 32 MSBs of the u64 value
    let cleared_u64_value = u64_value & ((1u64 << 32) - 1);

    // Combine the cleared u64 value with the shifted u64 value to insert using bitwise OR
    cleared_u64_value | shifted_u64_value_to_insert
}

fn main() {
    let i32_value: i32 = -20;
    let u64_value: u64 = i32_value as u64;

    println!("i32 value: {0:032b} {0}", i32_value);
    println!("u64 value: {0:064b} {0}", u64_value);
    let i32_value=!i32_value;
    let u64_value=!u64_value;
    println!("Their NOT values:");
    println!("i32 value: {0:032b} {0}", i32_value);
    println!("u64 value: {0:064b} {0}", u64_value);


    let u64_value: u64 = 0x6543210987654321; // Example u64 value with all bits set
    let i32_value: i32 = 0x12345678;         // Example i32 value

    let result = overwrite_msb_u64_with_i32(u64_value, i32_value);
    println!("before: {:08X} this will overwrite", i32_value);
    println!("before: {:016X} this", u64_value);
    println!("Result: {:016X}", result); // Print the result in hexadecimal format
}

