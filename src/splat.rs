use std::{ops::Index, process::Output};

use euclid::{Point2D, Transform2D, point2};
use image::{Rgb32FImage, Rgb};
use rand::random_range;

use crate::util::{Unit, to_normalized_pixels_mut, to_normalized_pixels_zipped};

#[derive(PartialEq)]
pub struct Splat {
    pub color: Rgb<f32>,
    pub alpha: f32,
    pub inverse_transform: Transform2D<f32, Unit, Unit>
}

impl Splat {
    pub fn to_array(&self) -> [f32; 10] {
        let col_arr = self.color.0;
        let mat_arr = self.inverse_transform.to_array();
        
        return [
            col_arr[0], col_arr[1], col_arr[2],
            self.alpha,
            mat_arr[0], mat_arr[1], mat_arr[2], mat_arr[3], mat_arr[4], mat_arr[5]
        ];
    }

    pub fn from_indexable<T>(vals: &T) -> Splat 
    where T: Index<usize, Output = f32>
    {
        return Splat { 
            color: Rgb([vals[0], vals[1], vals[2]]), 
            alpha: vals[3], 
            inverse_transform: Transform2D::from_array([vals[4], vals[5], vals[6], vals[7], vals[8], vals[9]])
        }
    }

    pub fn initialize_randomly() -> Splat {
        let color_rng = |_| random_range::<f32, _>(0.0 .. 1.0);
        let alpha_rng = |_| random_range::<f32, _>(0.5 .. 1.0);
        let matrix_rng = |_| random_range::<f32, _>(-1.0 .. 1.0);

        return Splat {
            color: Rgb::<f32>(core::array::from_fn(color_rng)),
            alpha: alpha_rng(()),
            inverse_transform: Transform2D::from_array(core::array::from_fn(matrix_rng))
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

    pub fn error(&self, goal_img: &Rgb32FImage, curr_img: &Rgb32FImage) -> f32 {
        let mut error: f32 = 0.0;
        
        for (pos, goal_pixel, curr_pixel) in to_normalized_pixels_zipped(goal_img, curr_img) {
            let blend_factor = self.point_blend_factor(&pos);

            for i in 0..3 {
                let goal_channel = goal_pixel.0[i];
                let curr_channel = curr_pixel.0[i];
                let splat_channel = self.color.0[i];

                let blended_channel = (1.0 - blend_factor) * (curr_channel) + blend_factor * (splat_channel);

                error += (blended_channel - goal_channel).powi(2);
            }
        }

        return error;
    }
}