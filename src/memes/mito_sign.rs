use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mito_sign(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let text = texts.get(0).map(|s| s.as_str()).unwrap_or("这是你？");
        let min_font_size = 12.0;
        let max_font_size = 72.0;
        let frame = load_image(format!("mito_sign/0.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].square().resize_exact((189, 197));
        canvas.draw_image(&img, (14, 420), None);
        canvas.draw_image(&frame, (0, 0), None);
        let rect = IRect::from_ltrb(66, 0, 564, 146);
        canvas.draw_text_area_auto_font_size(rect, text, min_font_size, max_font_size, None);
        Ok(surface.image_snapshot().resize_fit((310, 337), Fit::Cover))
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.05,
        },
        None,
    )
}

register_meme!(
    "mito_sign",
    mito_sign,
    min_texts = 0,
    max_texts = 1,
    min_images = 1,
    max_images = 1,
    keywords = &["美兔举猪"],
    date_created = local_date(2025, 12, 14),
    date_modified = local_date(2025, 12, 14),
);
