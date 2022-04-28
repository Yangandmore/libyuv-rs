use std::fs;

use nv21::convert::*;


#[test]
fn test_nv21_to_i420() {
    let path = "/Users/yangshiqu/Documents/IdeWorkSpaces/RustWorkSpace/libyuv-rs/img/yuv/lena_256x256_nv21.yuv";
    
    let file = fs::read(path).unwrap();
    const wi
    dth: usize = 256;
    const height: usize = 256;
    const count: usize = width * height;
    const src_stride_y: usize = width * height;
    const src_stride_uv: usize = count >> 1;
    const dst_stride_y: usize = width * height;
    const dst_stride_u: usize = count >> 2;
    const dst_stride_v: usize = count >> 2;
    const dst_count: usize = dst_stride_y + dst_stride_u + dst_stride_v;

    let src_y = &file[..((width * height) as usize)];
    let src_vu = &file[((width * height) as usize)..];

    println!("len: {}", file.len());
    println!("y len: {}", src_y.len());
    println!("vu len: {}", src_vu.len());

    let data = nv21_to_i420::<
        src_stride_y, src_stride_uv,
        dst_stride_y, dst_stride_u, dst_stride_v,
        width, height,
        dst_count
    >(src_y, src_vu);
    match data {
        Some(dst) => {
            let new_path = "/Users/yangshiqu/Documents/IdeWorkSpaces/RustWorkSpace/libyuv-rs/out/yuv/lena_256x256_i420.yuv";
            match fs::write(new_path, dst) {
                Ok(_) => {
                    println!("ok");       
                },
                Err(_) => {
                    println!("err");
                }
            }
        },
        None => {
            println!("err1")
        }
    }

}

#[test]
fn test_nv21_to_argb() {
    
}

#[test]
fn test_nv21_to_rgb565() {

}
