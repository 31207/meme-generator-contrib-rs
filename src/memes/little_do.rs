use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn little_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("little_do/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let self_head = images[0].circle().resize_exact((21, 21));
        let user_head = images[1].circle().resize_exact((21, 21)).rotate_crop(-90.0);
        canvas.draw_image(&self_head, (40, 4), None);
        canvas.draw_image(&user_head, (6, 46), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 7,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "little_do",
    little_do,
    min_images = 2,
    max_images = 2,
    keywords = &["小撅", "轻撅"],
    date_created = local_date(2024, 7, 12),
    date_modified = local_date(2024, 7, 12),
);
