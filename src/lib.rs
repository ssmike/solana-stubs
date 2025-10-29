#![cfg_attr(target_os="solana", feature(asm_experimental_arch))]
use core::arch::asm;

macro_rules! syscall_enter {
    () => {
        unsafe {
            asm! ("
                lddw r0, 0x2FFFF9EC0
                stxdw [r0 + 0], r1
                stxdw [r0 + 8], r2
                stxdw [r0 + 16], r3
                stxdw [r0 + 24], r4
                stxdw [r0 + 32], r5
                ")
        }
    }
}

macro_rules! syscall_exit {
    ($code: expr) => {
        unsafe {
            asm! ("
                mov64 r0, {ret}
                lddw r9, 0x2FFFF9EC0
                ldxdw r1, [r9 + 0]
                ldxdw r2, [r9 + 8]
                ldxdw r3, [r9 + 16]
                ldxdw r4, [r9 + 24]
                ldxdw r5, [r9 + 32]
                exit
                ",
                ret = in(reg) $code)
        }
    }
}

#[no_mangle]
pub extern "C" fn sol_log_(_dst: *mut u8, _addr: *const u8, _len: u64) -> u64 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn sol_memcpy_(dst: *mut u8, src: *const u8, n: u64) {
    syscall_enter!();

    for i in 0..n {
        *dst.offset(i as isize) = *src.offset(i as isize);
    }

    syscall_exit!(0);
}

#[no_mangle]
pub unsafe extern "C" fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32) {
    syscall_enter!();

    let mut cmpresult: i32 = 0;
    for i in 0..n as usize {
        let i = i as u64;
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            cmpresult = (a as i32).saturating_sub(b as i32);
            break;
        };
    }
    *result = cmpresult;

    syscall_exit!(0);
}

#[unsafe(no_mangle)]
pub unsafe fn entrypoint(_argc: usize) {
}
