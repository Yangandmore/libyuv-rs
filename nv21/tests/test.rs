use std::fs;

use nv21::convert::*;


#[test]
fn test_nv21_to_i420() {
    let path = "/Users/yangshiqu/Documents/IdeWorkSpaces/RustWorkSpace/libyuv-rs/img/yuv/lena_256x256_nv21.yuv";
    
    let file = fs::read(path).unwrap();
    const width: usize = 256;
    const height: usize = 256;
    const count: usize = width * height;
    let src_y = &file[..((width * height) as usize)];
    let src_vu = &file[((width * height) as usize)..];

    println!("len: {}", file.len());
    println!("y len: {}", src_y.len());
    println!("vu len: {}", src_vu.len());

    // NV21_to_I420::<width, height, count>(src_y, src_vu);
}

#[test]
fn test_nv21_to_argb() {
    
}

#[test]
fn test_nv21_to_rgb565() {

}
