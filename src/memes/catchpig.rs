use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    shortcut,
    tools::{load_image, local_date, new_surface},
    {image::Fit, image::ImageExt},
};

use crate::{options::NoOptions, register_meme};

fn catchpig(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let man_locs = [
        (193, 24),
        (191, 21),
        (191, 22),
        (189, 18),
        (187, 28),
        (188, 26),
        (186, 29),
        (183, 33),
        (180, 40),
        (174, 40),
        (170, 43),
        (161, 46),
        (161, 43),
        (150, 43),
        (143, 39),
        (139, 37),
        (135, 35),
        (133, 37),
        (134, 37),
        (132, 39),
        (129, 43),
        (129, 41),
        (129, 42),
        (127, 45),
        (128, 51),
        (129, 51),
        (129, 57),
        (128, 58),
        (128, 62),
        (130, 62),
        (132, 60),
        (132, 62),
        (139, 71),
        (133, 47),
        (152, 55),
        (157, 43),
        (162, 48),
        (168, 49),
        (170, 51),
        (169, 27),
        (176, 17),
        (188, 17),
        (197, 13),
        (208, 10),
        (212, 7),
        (214, 7),
        (216, 10),
        (214, 8),
        (218, 7),
        (224, 10),
        (223, 6),
        (239, 6),
        (261, 6),
    ];
    let pig_locs = [
        (26, 74),
        (25, 76),
        (26, 80),
        (27, 82),
        (29, 83),
        (29, 86),
        (31, 88),
        (27, 92),
        (27, 94),
        (28, 98),
        (28, 96),
        (24, 99),
        (26, 99),
        (24, 100),
        (23, 97),
        (23, 94),
        (23, 95),
        (22, 89),
        (25, 94),
        (28, 94),
        (28, 96),
        (29, 96),
        (30, 93),
        (31, 87),
        (30, 88),
        (32, 91),
        (32, 94),
        (38, 89),
        (45, 94),
        (43, 97),
        (45, 95),
        (42, 98),
        (46, 104),
        (47, 99),
        (67, 117),
        (60, 109),
        (66, 112),
        (84, 120),
        (83, 103),
        (79, 81),
        (97, 88),
        (93, 78),
        (99, 82),
        (102, 72),
        (120, 72),
        (125, 55),
        (116, 50),
        (122, 48),
        (136, 71),
        (179, 62),
        (207, 51),
        (204, 52),
        (210, 25),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let self_head = images[0].circle().resize_exact((64, 64));
        let user_head = images[1].circle().resize_exact((64, 64));
        let image = load_image(format!("catchpig/{i:02}.png"))?;
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&self_head, man_locs[i], None);
        canvas.draw_image(&user_head, pig_locs[i], None);
        Ok(surface.image_snapshot().resize_fit((100, 98), Fit::Cover))
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 53,
            duration: 4.5 / 53 as f32,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "catchpig",
    catchpig,
    min_images = 2,
    max_images = 2,
    keywords = &["抓猪"],
    date_created = local_date(2025, 12, 13),
    date_modified = local_date(2025, 12, 13),
);
