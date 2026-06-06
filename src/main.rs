use image::{ImageReader, Rgb32FImage, RgbImage, buffer::ConvertBuffer};

use crate::{optimize_splat::optimize_splat};

mod splat;
mod util;
mod optimize_splat;

fn main() {
    let goal_img = ImageReader::open("src/cropped_2401_small.png")
        .expect("Unable to get the requested image")
        .decode()
        .expect("Unable to decode the requested image")
        .to_rgb32f();

    let mut curr_img = Rgb32FImage::new(goal_img.width(), goal_img.height());

    let mut big_output_img = Rgb32FImage::new(1000, 1000);

    const SPLAT_COUNT: usize = 100;
    const TRIALS_PER_SPLAT: u64 = 1000;
    const INITIAL_VALUES_PER_SPLAT: u64 = 8;

    for i in 0..SPLAT_COUNT {
        let optimizations = (0..INITIAL_VALUES_PER_SPLAT).into_iter().map(|_| optimize_splat(&goal_img, &curr_img, TRIALS_PER_SPLAT));

        let best_optimization = optimizations.min_by(|x, y| x.1.total_cmp(&y.1)).unwrap();

        let best_splat = best_optimization.0;

        best_splat.apply(&mut curr_img);
        best_splat.apply(&mut big_output_img);
        ConvertBuffer::<RgbImage>::convert(&curr_img).save("progress_img.png").expect("Failed to save image");
        ConvertBuffer::<RgbImage>::convert(&big_output_img).save("big_output_img.png").expect("Failed to save image");
        println!("Completed Splat {}/{}", i + 1, SPLAT_COUNT)
    }

    ConvertBuffer::<RgbImage>::convert(&curr_img).save("output.png").expect("Failed to save image");
}
