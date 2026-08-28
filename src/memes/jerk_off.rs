use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn jerk_off(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let jerk = load_image("jerk_off/0.png")?;
    let jerk_w = jerk.width();
    let jerk_h = jerk.height();
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();

    let (frame_w, frame_h) = if img_w as f32 / img_h as f32 > jerk_w as f32 / jerk_h as f32 {
        (img_w * jerk_h / img_h, jerk_h)
    } else {
        (jerk_w, img_h * jerk_w / img_w)
    };

    let func = |i: usize, images: Vec<Image>| {
        let img = images[0].resize_fit((frame_w, frame_h), Fit::Cover);
        let jerk = load_image(format!("jerk_off/{i}.png"))?;
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&jerk, ((frame_w - jerk_w) / 2, frame_h - jerk_h), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 8,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "jerk_off",
    jerk_off,
    min_images = 1,
    max_images = 1,
    keywords = &["打胶"],
    date_created = local_date(2024, 8, 4),
    date_modified = local_date(2024, 8, 4),
);
