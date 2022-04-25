

/// 复制数据平面
pub fn copy_plane(
    src_y: &[u8],
    dst_y: &mut [u8],
) {
    for (index, item) in src_y.iter().enumerate() {
        dst_y[index] = *item;
    }
}

/// 支持NV12等UV通道功能。
/// 宽度和高度是平面尺寸（通常是半像素宽度）。
pub fn split_uv_plane(
    src_uv: &[u8],
    dst_u: &mut [u8],
    dst_v: &mut [u8],
    width: i32,
    height: i32,
) {
    
}