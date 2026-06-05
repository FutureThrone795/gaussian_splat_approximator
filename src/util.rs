use euclid::{Point2D, point2};
use image::{Rgb, Rgb32FImage};

// An empty struct for Matrices that expect a Src or Dst 
pub struct Unit;

// Returns an iterator of (pos, Rgba<f32>), where pos is normalized along the image's (width, height) and with an offset such that it is in the centre of the pixel
pub fn to_normalized_pixels_mut(img: &mut Rgb32FImage) -> impl Iterator<Item = (Point2D<f32, Unit>, &mut Rgb<f32>)> {
    let width = img.width() as f32;
    let height = img.height() as f32;

    return img.enumerate_pixels_mut().map(
        move |(x, y, pixel)| {
            let x_normalized = (x as f32 + 0.5) / width;
            let y_normalized = (y as f32 + 0.5) / height;

            return (point2::<f32, Unit>(x_normalized, y_normalized), pixel);
    });
}