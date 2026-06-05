use euclid::{Transform2D, vec2};
use image::{Rgb, Rgb32FImage, RgbImage, buffer::ConvertBuffer};

use crate::splat::Splat;

mod splat;
mod util;

fn main() {
    let mut img = Rgb32FImage::new(100, 100);

    let red_splat = Splat {
        color: Rgb([1.0, 0.0, 0.0]),
        alpha: 1.0,
        transform: Transform2D::identity().then_scale(0.1, 0.2).then_rotate(euclid::Angle { radians: 0.5 }).then_translate(vec2(0.5, 0.5))
    };

    let blue_splat = Splat {
        color: Rgb([0.0, 0.2, 1.0]),
        alpha: 0.5,
        transform: Transform2D::identity().then_scale(0.4, 0.2).then_rotate(euclid::Angle { radians: 0.5 }).then_translate(vec2(0.75, 0.75))
    };

    red_splat.apply(&mut img).expect("Failed to apply red_splat");
    blue_splat.apply(&mut img).expect("Failed to apply blue_plat");

    ConvertBuffer::<RgbImage>::convert(&img).save("output.png").expect("Failed to save image");
}
