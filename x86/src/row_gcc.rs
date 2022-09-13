/// 复制数据平面-sse2
#[cfg(any(target_arch = "x86_64", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
pub unsafe fn copy_row_sse2(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    let mut src_y = src_y;

    #[cfg(target_arch="x86_64")]
    use std::arch::x86_64::*;
    #[cfg(target_arch="x86")]
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

/// 复制数据平面-sse3
#[cfg(any(target_arch = "x86_64", target_arch = "x86_64"))]
#[target_feature(enable = "sse3")]
pub unsafe fn copy_row_sse3(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    let mut src_y = src_y;

    #[cfg(target_arch="x86_64")]
    use std::arch::x86_64::*;
    #[cfg(target_arch="x86")]
    use std::arch::x86::*;

    let mut i = 0_isize;
    while src_y.len() >= 16 {
        // 加载数据
        let invec = _mm_lddqu_si128(src_y.as_ptr() as *const _);
    
        // 数据移动到dst
        _mm_storeu_si128(
            dst_y.as_mut_ptr().offset(i) as *mut _,
            invec,
        );
        
        src_y = &src_y[16..];
        i += 16;
    }
}

/// 复制数据平面-avx
#[cfg(any(target_arch = "x86_64", target_arch = "x86_64"))]
#[target_feature(enable = "avx")]
pub unsafe fn copy_row_avx(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    let mut src_y = src_y;

    #[cfg(target_arch="x86_64")]
    use std::arch::x86_64::*;
    #[cfg(target_arch="x86")]
    use std::arch::x86::*;

    let mut i = 0_isize;
    while src_y.len() >= 32 {
        // 加载数据
        let invec = _mm256_lddqu_si256(src_y.as_ptr() as *const _);
    
        // 数据移动到dst
        _mm256_storeu_si256(
            dst_y.as_mut_ptr().offset(i) as *mut _,
            invec,
        );
        
        src_y = &src_y[32..];
        i += 32;
    }
}

