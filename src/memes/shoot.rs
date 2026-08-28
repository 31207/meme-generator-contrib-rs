use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn shoot(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let fluid = load_image(format!("shoot/{i:02}.png"))?;
        let img = images[0].resize_fit(fluid.dimensions(), Fit::Cover);
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&fluid, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 13,
            duration: 0.15,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "shoot",
    shoot,
    min_images = 1,
    max_images = 1,
    keywords = &["射", "🐍"],
    date_created = local_date(2024, 8, 19),
    date_modified = local_date(2024, 8, 19),
);
