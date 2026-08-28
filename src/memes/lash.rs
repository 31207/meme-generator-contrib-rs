use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn lash(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [
        (84, 25),
        (87, 23),
        (87, 27),
        (86, 28),
        (62, 26),
        (59, 28),
        (76, 20),
        (85, 24),
        (80, 23),
    ];
    let user_locs = [
        (12, 69),
        (15, 66),
        (14, 67),
        (15, 66),
        (17, 67),
        (14, 63),
        (21, 56),
        (15, 62),
        (17, 69),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("lash/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let self_head = images[0].circle().resize_exact((22, 22));
        let user_head = images[1].circle().resize_exact((22, 22)).rotate_crop(-30.0);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&user_head, user_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 9,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "lash",
    lash,
    min_images = 2,
    max_images = 2,
    keywords = &["鞭笞", "鞭打"],
    date_created = local_date(2024, 7, 23),
    date_modified = local_date(2024, 7, 23),
);
