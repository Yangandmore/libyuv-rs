/// 复制数据平面
pub fn copy_row_rs(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    for (index, item) in src_y.iter().enumerate() {
        dst_y[index] = *item;
    }
}

/// 支持NV12等UV通道功能。
/// 宽度和高度是平面尺寸（通常是半像素宽度）。
pub fn split_uv_plane_rs(
    src_uv: &[u8],
    dst_uv: &mut [u8],
    width: usize,
    height: usize
) {
    let mut src_index = 0;
    let mut dst_index = 0;
    let count = width * height;
    let helf_count = count >> 1;
    loop {
        if dst_index >= helf_count {
            break;
        }

        // U
        dst_uv[dst_index] = src_uv[src_index];
        // V
        dst_uv[dst_index + helf_count] = src_uv[src_index + 1];

        src_index += 2;
        dst_index += 1;
    };
}

/// 支持NV21等VU通道功能。
/// 宽度和高度是平面尺寸（通常是半像素宽度）。
pub fn split_vu_plane_rs(
    src_vu: &[u8],
    dst_uv: &mut [u8],
    width: usize,
    height: usize
) {
    let mut src_index = 0;
    let mut dst_index = 0;
    let count = width * height;
    let helf_count = count >> 1;

    loop {
        if dst_index >= helf_count {
            break;
        }
        
        // U
        dst_uv[dst_index] = src_vu[src_index + 1];
        // V
        dst_uv[dst_index + helf_count] = src_vu[src_index];

        src_index += 2;
        dst_index += 1;
    };
}
