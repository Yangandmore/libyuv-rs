use libyuv_rs::planar_function::{self, copy_plane, split_vu_plane};



/// nv21 to I420
pub fn nv21_to_i420<
    const src_stride_y: usize,  const src_stride_uv: usize,
    const dst_stride_y: usize, const dst_stride_u: usize, const dst_stride_v: usize, 
    const width: usize, const height: usize,
    const dst_count: usize
>(
    src_y: &[u8], src_vu: &[u8],
) -> Option<[u8; dst_count]> {
    let half_height = (height + 1) >> 1;

    // TODO: 检查格式
    if src_y.len() <= 0 || src_vu.len() <= 0 || width <= 0 || height == 0 {
        return None;
    }

    // TODO: 负高度则图片反转

    // TODO: yuv各项计算
    let mut dst: [u8; dst_count] = [0; dst_count];

    // TODO: 根据环境不同，进行编译

    // 1.copy Y
    copy_plane(src_y, &mut dst[..dst_stride_y]);
    // 2.copy vu
    split_vu_plane(src_vu, &mut dst[dst_stride_y..], width, half_height);

    Some(dst)
}

