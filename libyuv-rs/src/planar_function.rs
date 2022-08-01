use crate::row_common::copy_row_rs;

/// 复制数据平面
pub fn copy_plane(src_y: &[u8], dst_y: &mut [u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx") {
            println!("go avx");
            unsafe { crate::row_gcc::copy_row_avx(src_y, dst_y) };
            return ;
        }
        if is_x86_feature_detected!("sse3") {
            println!("go sse3");
            unsafe { crate::row_gcc::copy_row_sse3(src_y, dst_y) };
            return ;
        }
        if is_x86_feature_detected!("sse2") {
            println!("go sse2");
            unsafe { crate::row_gcc::copy_row_sse2(src_y, dst_y) };
            return ;
        }
    }

    #[cfg(all(target_arch = "aarch64"))]
    {
        println!("go neon");
        // unsafe { crate::row_neon::copy_row_neon(src_y, dst_y) };
        return ;
    }

    #[cfg(all(target_arch = "arm"))]
    {
        println!("go arm");
        // unsafe { crate::row_neon::copy_row_neon(src_y, dst_y) };
        return ;
    }

    {
        println!("default");
        copy_row_rs(src_y, dst_y);
        return ;
    }
}
