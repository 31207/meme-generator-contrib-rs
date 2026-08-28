use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn behead(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (80, 72, 0),
        (83, 73, 0),
        (82, 73, 0),
        (78, 73, 0),
        (72, 74, 0),
        (72, 75, 0),
        (73, 76, 0),
        (73, 76, 0),
        (73, 76, 0),
        (74, 76, 0),
        (74, 76, 0),
        (70, 73, -12),
        (61, 62, -25),
        (49, 40, -45),
        (46, 30, -65),
        (50, 35, -85),
        (39, 34, -105),
        (19, 45, -135),
        (9, 91, -155),
        (6, 161, -175),
        (-4, 248, -180),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("behead/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, angle) = locs[i];
        let img = images[0]
            .square()
            .resize_exact((75, 75))
            .rotate(angle as f32);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 21,
            duration: 0.05,
        },
        None,
    )
}

register_meme!(
    "behead",
    behead,
    min_images = 1,
    max_images = 1,
    keywords = &["砍头", "斩首"],
    date_created = local_date(2023, 7, 1),
    date_modified = local_date(2023, 7, 1),
);
