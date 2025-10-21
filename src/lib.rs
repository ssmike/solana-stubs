#[unsafe(no_mangle)]
pub unsafe fn sol_log_(_dst: *mut u8, _src: *const u8, _n: u64) -> u64 {
    0
}

#[unsafe(no_mangle)]
pub unsafe fn sol_memcpy_(dst: *mut u8, src: *const u8, n: u64) -> u64 {
    for i in 0..n {
        *dst.offset(i as isize) = *src.offset(i as isize);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32) -> u64 {
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

    0
}

#[unsafe(no_mangle)]
pub unsafe fn entrypoint(_argc: usize) {
}
