use std::fs;

use libyuv_rs::convert::{nv21_to_i420, nv12_to_i420};

#[test]
fn test_cpu_id() {

}

#[test]
fn test_nv21_to_i420() {
    let path = "/Users/yangshiqu/Documents/IdeWorkSpaces/RustWorkSpace/libyuv-rs/img/yuv/lena_256x256_nv21.yuv";
    
    let file = fs::read(path).unwrap();
    const width: usize = 256;
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

    let now = std::time::Instant::now();

    let data = nv21_to_i420::<
        src_stride_y, src_stride_uv,
        dst_stride_y, dst_stride_u, dst_stride_v,
        width, height,
        dst_count
    >(src_y, src_vu);

    println!("nv21_to_i420:{:?}", now.elapsed().as_millis());

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
fn test_nv12_to_i420() {
    let path = "/Users/yangshiqu/Documents/IdeWorkSpaces/RustWorkSpace/libyuv-rs/img/yuv/lena_256x256_nv12.yuv";
    
    let file = fs::read(path).unwrap();
    const width: usize = 256;
    const height: usize = 256;
    const count: usize = width * height;
    const src_stride_y: usize = width * height;
    const src_stride_uv: usize = count >> 1;
    const dst_stride_y: usize = width * height;
    const dst_stride_u: usize = count >> 2;
    const dst_stride_v: usize = count >> 2;
    const dst_count: usize = dst_stride_y + dst_stride_u + dst_stride_v;

    let src_y = &file[..((width * height) as usize)];
    let src_uv = &file[((width * height) as usize)..];

    let now = std::time::Instant::now();
    let data = nv12_to_i420::<
        src_stride_y, src_stride_uv,
        dst_stride_y, dst_stride_u, dst_stride_v,
        width, height,
        dst_count
    >(src_y, src_uv);
    println!("nv21_to_i420:{:?}", now.elapsed().as_millis());
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
