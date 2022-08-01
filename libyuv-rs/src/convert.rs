use crate::{planar_function::copy_plane, row_common::{split_vu_plane_rs, split_uv_plane_rs}};

/// nv21 to I420
pub fn nv21_to_i420<
    const SRC_STRIDE_Y: usize,  const SRC_STRIDE_UV: usize,
    const DST_STRIDE_Y: usize, const DST_STRIDE_U: usize, const DST_STRIDE_V: usize, 
    const WIDTH: usize, const HEIGHT: usize,
    const DST_COUNT: usize
>(src_y: &[u8], src_vu: &[u8],) -> Option<[u8; DST_COUNT]> {
    let half_height = (HEIGHT + 1) >> 1;

    if src_y.len() <= 0 || src_vu.len() <= 0 || WIDTH <= 0 || HEIGHT == 0 {
        return None;
    }

    // 返回数据
    let mut dst: [u8; DST_COUNT] = [0; DST_COUNT];

    // 1.copy Y
    copy_plane(src_y, &mut dst[..DST_STRIDE_Y]);

    // 获取适当的闭包
    let split_vu_plane = split_vu_plane_rs;


    // 2.copy vu
    split_vu_plane(src_vu, &mut dst[DST_STRIDE_Y..], WIDTH, half_height);

    Some(dst)
}

/// nv12 to I420
pub fn nv12_to_i420<
    const SRC_STRIDE_Y: usize,  const SRC_STRIDE_UV: usize,
    const DST_STRIDE_Y: usize, const DST_STRIDE_U: usize, const DST_STRIDE_V: usize, 
    const WIDTH: usize, const HEIGHT: usize,
    const DST_COUNT: usize
>(src_y: &[u8], src_vu: &[u8],) -> Option<[u8; DST_COUNT]> {
    let half_height = (HEIGHT + 1) >> 1;

    if src_y.len() <= 0 || src_vu.len() <= 0 || WIDTH <= 0 || HEIGHT == 0 {
        return None;
    }

    let mut dst: [u8; DST_COUNT] = [0; DST_COUNT];

    let split_uv_plane = split_uv_plane_rs;
    
    // 1.copy Y
    copy_plane(src_y, &mut dst[..DST_STRIDE_Y]);

    // 2.copy vu
    split_uv_plane(src_vu, &mut dst[DST_STRIDE_Y..], WIDTH, half_height);

    Some(dst)
}
