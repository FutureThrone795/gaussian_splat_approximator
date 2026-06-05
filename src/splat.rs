use euclid::{Transform2D, point2};
use image::{Rgb32FImage, Rgb};

use crate::util::{Unit, to_normalized_pixels_mut};

pub struct Splat {
    pub color: Rgb<f32>,
    pub alpha: f32,
    pub inverse_transform: Transform2D<f32, Unit, Unit>
}

#[derive(Debug)]
pub enum SplatError {
    NotInversible
}

impl Splat {
    pub fn apply(&self, img: &mut Rgb32FImage) -> Result<(), SplatError> {
        for (pos, pixel) in to_normalized_pixels_mut(img) {
            let transformed_pos = self.inverse_transform.transform_point(pos);

            let dist_from_origin = transformed_pos.distance_to(point2(0.0, 0.0));
            let gaussian_factor = f32::exp(-dist_from_origin.powi(2));
            let blend_factor = gaussian_factor * self.alpha;

            for (current_channel, splat_channel) in pixel.0.iter_mut().zip(&self.color.0) {
                *current_channel = (1.0 - blend_factor) * (*current_channel) + blend_factor * (*splat_channel);
            }
        }

        return Ok(());
    }
}