use skia_safe::{Image, Matrix, Point, font_style::Width, surface};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn double_heartbeat(x: f32) -> f32 {
    // 基础配置
    let peak_1_pos = 0.25;  // 第一个心跳位置
    let peak_2_pos = 0.65;  // 第二个心跳位置
    let sharpness = 0.011; // 宽度参数（w），越小越尖锐
    let epsilon = 0.001;   // 确保恒大于零的基底高度

    // 高斯函数：exp(-(x - center)^2 / width)
    let pulse1 = (-(x - peak_1_pos).powi(2) / sharpness).exp();
    let pulse2 = (-(x - peak_2_pos).powi(2) / sharpness).exp();

    // 叠加并确保恒大于零
    pulse1 + pulse2 + epsilon
}

fn jumpscare(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let mut pic = images[0].clone();
        if images[0].width() > 400 || images[0].height() > 400 {
            pic = images[0].resize_fit((400, 380), Fit::Contain);
        }
        let mut surface = pic.to_surface();
        let canvas = surface.canvas();
        canvas.clear(skia_safe::Color::TRANSPARENT);
        let (w, h) = (pic.width() as f32, pic.height() as f32);
        let src = [
            Point::new(0.0, 0.0),
            Point::new(w, 0.0),
            Point::new(w, h),
            Point::new(0.0, h),
        ];

        let offset = h * 0.2; // 缩进量
        let dst = [
            Point::new(offset, offset * 1.5),     // 左上向右移
            Point::new(w - offset, offset * 1.5), // 右上向左移
            Point::new(w, h - offset / 2.0),                     // 右下不动
            Point::new(0.0, h - offset / 2.0),                   // 左下不动
        ];
        let matrix = Matrix::from_poly_to_poly(&src, &dst).unwrap_or_default();
        canvas.concat(&matrix);
        let factor = 1.0 + double_heartbeat(i as f32 / 12.0) * 2.0;
        let width = pic.width() as f32 * factor;
        let width = width as i32;
        canvas.draw_image(
            &pic.resize_fit((width, width), Fit::Cover),
            Point::new(w * (1.0 - factor) / 2.0, h * (1.0 - factor) / 2.0),
            None,
        );
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 12,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "jumpscare",
    jumpscare,
    min_images = 1,
    max_images = 1,
    keywords = &["跳吓", "惊吓"],
    date_created = local_date(2026, 1, 16),
    date_modified = local_date(2026, 1, 16),
);
