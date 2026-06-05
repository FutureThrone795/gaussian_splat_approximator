use euclid::Transform2D;
use image::Rgb;
use optimizer::{parameter::Parameter, prelude::{FloatParam, Trial}, sampler::CompletedTrial};

use crate::Splat;

pub struct SplatTrialParams {
    r: FloatParam,
    g: FloatParam,
    b: FloatParam,

    a: FloatParam,

    m11: FloatParam,
    m12: FloatParam,
    m21: FloatParam,
    m22: FloatParam,
    m31: FloatParam,
    m32: FloatParam
}

impl SplatTrialParams {
    pub fn new() -> SplatTrialParams {
        return SplatTrialParams { 
            r: FloatParam::new(0.0, 1.0).name("r"),
            g: FloatParam::new(0.0, 1.0).name("g"),
            b: FloatParam::new(0.0, 1.0).name("b"),

            a: FloatParam::new(0.0, 1.0).name("b"),

            m11: FloatParam::new(-10.0, 10.0).name("m11"),
            m12: FloatParam::new(-10.0, 10.0).name("m12"),
            m21: FloatParam::new(-10.0, 10.0).name("m21"),
            m22: FloatParam::new(-10.0, 10.0).name("m22"),
            m31: FloatParam::new(-10.0, 10.0).name("m31"),
            m32: FloatParam::new(-10.0, 10.0).name("m32") 
        }
    }

    fn color_params(&self) -> [&FloatParam; 3] {
        return [&self.r, &self.g, &self.b];
    }
    fn alpha_param(&self) -> &FloatParam {
        return &self.a;
    }
    fn matrix_params(&self) -> [&FloatParam; 6] {
        return [&self.m11, &self.m12, &self.m21, &self.m22, &self.m31, &self.m32];
    }

    pub fn suggest_splat(&self, trial: &mut Trial) -> Splat {
        let mut suggest_splat_fn = |param: &FloatParam| param.suggest(trial).unwrap() as f32;

        return Splat {
            color: 
                Rgb(
                    self.color_params().map(&mut suggest_splat_fn)
                ),
            alpha: 
                suggest_splat_fn(self.alpha_param()),
            inverse_transform: 
                Transform2D::from_array(
                    self.matrix_params().map(&mut suggest_splat_fn)
                )
        };
    }

    pub fn best_splat(&self, completed_trial: &CompletedTrial<f32>) -> Splat {
        let mut best_splat_fn = |param| completed_trial.get(param).unwrap() as f32;

        return Splat {
            color: 
                Rgb(
                    self.color_params().map(&mut best_splat_fn)
                ),
            alpha: 
                best_splat_fn(self.alpha_param()),
            inverse_transform: 
                Transform2D::from_array(
                    self.matrix_params().map(&mut best_splat_fn)
                )
        };
    }
}