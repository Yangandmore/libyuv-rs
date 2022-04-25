use libyuv_rs::planar_function::{self, copy_plane};



/// nv21 to I420
pub fn NV21_to_I420<const width: usize, const height: usize, const count: usize>(
    src_y: &[u8], src_vu: &[u8],
) -> Option<[u8; count]> {
    let half_width = (width + 1) >> 1;
    let half_height = (height + 1) >> 1;
    let half_count = count >> 1;

    // TODO: 检查格式
    if src_y.len() <= 0 || src_vu.len() <= 0 || width <= 0 || height == 0 {
        return None;
    }

    // TODO: 负高度则图片反转

    // TODO: yuv各项计算
    let mut dst: [u8; count] = [0; count];
    // TODO: 根据环境不同，进行编译
    // 1.copy Y
    copy_plane(src_y, &mut dst[..count]);
    // 2.copy vu
    

    Some(dst)
}

