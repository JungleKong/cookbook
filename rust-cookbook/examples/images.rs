use image::{self, DynamicImage, ImageBuffer};

use std::fs::File;
use std::io::Write;

fn main() {
    // camera_0.raw 1920x1280 (not certain `UYUV or YUYV`)
    const  IMAGE_BYTE: &'static [u8] = include_bytes!("../../assets/images/camera_0.raw");

    // 将buffer保存为图片
    match image::save_buffer("../assets/images/test.png", IMAGE_BYTE, 1920, 1280, image::ColorType::L16) {
        Ok(_) => println!("save success"),
        Err(e) => println!("save error: {}", e),
    }

    // 从raw中加载一张图片，因为raw不包含宽高信息，所以需要手动指定宽高
    // 同时因为raw的格式为UYUV，image库不支持，所以需要转换为LumaA，但是结果不正确，todo!()
    let a = ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(1920, 1280, IMAGE_BYTE.into()).unwrap();
    let img = DynamicImage::ImageLumaA8(a.clone());
    img.save("../assets/images/test2.png").unwrap();

    // 直接打开图片
    let img = image::open("../assets/images/test.png").unwrap();

    // 保存图片buffer为raw，保存结果不包含宽高信息
    let mut file = File::create("../assets/images/camera.raw").unwrap();
    file.write_all(img.as_bytes()).unwrap();
}
