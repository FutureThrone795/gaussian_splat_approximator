use std::marker::PhantomData;

use euclid::{Point2D, Transform2D, point2};
use image::{Rgb32FImage, Rgb};

use crate::util::{Unit, to_normalized_pixels_mut, to_normalized_pixels};

#[derive(PartialEq)]
pub struct Splat {
    pub color: Rgb<f32>,
    pub alpha: f32,
    pub inverse_transform: Transform2D<f32, Unit, Unit>
}

impl Splat {
    pub fn from_values(r: f32, g: f32, b: f32, a: f32, m11: f32, m12: f32, m21: f32, m22: f32, m31: f32, m32: f32) -> Splat {
        return Splat { 
            color: Rgb([r, g, b]), 
            alpha: a, 
            inverse_transform: Transform2D { m11, m12, m21, m22, m31, m32, _unit: PhantomData }
        }
    }

    pub fn point_blend_factor(&self, pos: &Point2D<f32, Unit>) -> f32 {
        let transformed_pos = self.inverse_transform.transform_point(*pos);
        let dist_from_origin = transformed_pos.distance_to(point2(0.0, 0.0));
        let gaussian_factor = f32::exp(-dist_from_origin.powi(2));
        return gaussian_factor * self.alpha;
    }

    pub fn apply(&self, img: &mut Rgb32FImage) {
        for (pos, pixel) in to_normalized_pixels_mut(img) {
            let blend_factor = self.point_blend_factor(&pos);

            for (current_channel, splat_channel) in pixel.0.iter_mut().zip(&self.color.0) {
                *current_channel = (1.0 - blend_factor) * (*current_channel) + blend_factor * (*splat_channel);
            }
        }
    }

    pub fn error(&self, img: &Rgb32FImage) -> f32 {
        let mut error: f32 = 0.0;
        
        for (pos, pixel) in to_normalized_pixels(img) {
            let blend_factor = self.point_blend_factor(&pos);

            for (current_channel, splat_channel) in pixel.0.iter().zip(&self.color.0) {
                let new_color = (1.0 - blend_factor) * (*current_channel) + blend_factor * (*splat_channel);

                error += (new_color - blend_factor).powi(2);
            }
        }

        return error;
    }
}