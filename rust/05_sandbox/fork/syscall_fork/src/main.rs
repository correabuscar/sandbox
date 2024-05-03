#![cfg(unix)]
use std::arch::asm;

fn fork_syscall() -> isize {
    let result: isize;
    unsafe {
        asm!(
            "syscall",
            inout("rax") 57_i64 => result, // 57 is the syscall number for fork on x86_64
            lateout("rcx") _, // Preserve rcx register
            lateout("r11") _, // Preserve r11 register
            options(nomem, preserves_flags),
        );
    }
    result
}

fn main() {
    let result = fork_syscall();

    match result {
        0 => {
            // Child process
            println!("This is the child process");
            // Additional logic for child process
        }
        -1 => {
            // Error
            println!("Fork failed");
        }
        _ => {
            // Parent process
            println!("This is the parent process");
            // Additional logic for parent process
        }
    }
}

