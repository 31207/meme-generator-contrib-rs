use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    decoder::CodecExt,
    encoder::{GifEncoder, encode_png},
    tools::local_date,
};

use crate::{options::NoOptions, register_meme};

fn reverse(mut images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let input = &mut images[0];
    let frame_count = input.codec.get_frame_count();
    if frame_count <= 1 {
        return encode_png(input.image.clone());
    }

    let mut durations = Vec::with_capacity(frame_count);
    for i in 0..frame_count {
        let duration = input
            .codec
            .get_frame_info(i)
            .map(|frame_info| frame_info.duration as f32 / 1000.0)
            .unwrap_or(0.1);
        durations.push(duration);
    }

    let mut encoder = GifEncoder::new();
    for i in (0..frame_count).rev() {
        let frame = input.codec.get_frame(i)?;
        encoder.add_frame(frame, durations[i])?;
    }
    encoder.finish()
}

register_meme!(
    "reverse",
    reverse,
    min_images = 1,
    max_images = 1,
    keywords = &["倒放", "倒着放"],
    date_created = local_date(2026, 8, 28),
    date_modified = local_date(2026, 8, 28),
);
