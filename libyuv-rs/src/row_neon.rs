/// 复制数据平面-neon
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[target_feature(enable = "neon")]
pub unsafe fn copy_row_neon(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    let mut src_y = src_y;

    #[cfg(target_arch="arm")]
    use std::arch::x86_64::*;
    #[cfg(target_arch="aarch64")]
    use std::arch::x86::*;

    let mut i = 0_isize;
    while src_y.len() >= 16 {
        // 加载数据
        let invec = _mm_loadu_si128(src_y.as_ptr() as *const _);
    
        // 数据移动到dst
        _mm_storeu_si128(
            dst_y.as_mut_ptr().offset(i) as *mut _,
            invec,
        );
        
        src_y = &src_y[16..];
        i += 16;
    }
}
