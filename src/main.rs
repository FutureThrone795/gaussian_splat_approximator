use euclid::{Transform2D, vec2};
use image::{Rgb, Rgb32FImage, RgbImage, buffer::ConvertBuffer};

use crate::splat::Splat;

mod splat;
mod util;
mod optimize_splat;
mod splat_trial_params;

fn main() {
    let img_sizes = [5_u32, 20, 100, 1000];

    let mut imgs: Vec<(Rgb32FImage, u32)> = img_sizes.iter().map(|x| (Rgb32FImage::new(*x, *x), *x)).collect();

    let red_splat = Splat {
        color: Rgb([1.0, 0.0, 0.0]),
        alpha: 1.0,
        inverse_transform: Transform2D::identity().then_scale(0.1, 0.2).then_rotate(euclid::Angle { radians: 0.5 }).then_translate(vec2(0.5, 0.5)).inverse().unwrap()
    };

    let blue_splat = Splat {
        color: Rgb([0.0, 0.2, 1.0]),
        alpha: 0.5,
        inverse_transform: Transform2D::identity().then_scale(0.4, 0.2).then_rotate(euclid::Angle { radians: 0.25 }).then_translate(vec2(0.75, 0.75)).inverse().unwrap()
    };

    for (img, size) in &mut imgs {
        red_splat.apply(img);
        blue_splat.apply(img);

        ConvertBuffer::<RgbImage>::convert(img).save(format!("output{0}x{0}.png", size)).expect("Failed to save image");
    }
}
