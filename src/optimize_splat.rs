use std::fmt::Error;

use image::{Rgb32FImage};
use optimizer::{Direction, Study, prelude::Trial};

use crate::splat::Splat;
use crate::splat_trial_params::SplatTrialParams;

pub fn optimize_splat(img: &Rgb32FImage) -> Splat {
    let study: Study<f32> = Study::new(Direction::Minimize);

    let splat_params = SplatTrialParams::new();

    study.optimize(50, |trial: &mut Trial| {
        let splat = splat_params.suggest_splat(trial);

        return Ok::<f32, Error>(splat.error(img));
    }).unwrap();

    let best = study.best_trial().unwrap();
    return splat_params.best_splat(&best);
}