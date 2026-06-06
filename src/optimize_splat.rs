use argmin::{core::{CostFunction, Error, Executor}, solver::{self, neldermead::NelderMead}};
use euclid::{Transform2D, vec2};
use image::{Rgb, Rgb32FImage};

use crate::{splat::Splat, util::Unit};

struct OptimizeSplat<'a, 'b> {
    goal_img: &'a Rgb32FImage,
    curr_img: &'b Rgb32FImage
}

impl<'a, 'b> CostFunction for OptimizeSplat<'a, 'b> {
    type Param = Vec<f32>;
    type Output = f32;

    fn cost(&self, param: &Vec<f32>) -> Result<f32, Error> {
        return Ok(Splat::from_indexable(param).error(self.goal_img, self.curr_img));
    }
}

/// Returns (Splat, error)
pub fn optimize_splat(goal_img: &Rgb32FImage, curr_img: &Rgb32FImage, n_trials: u64) -> (Splat, f32) {
    let init_splat = Splat::initialize_randomly();
    let init_splat_arr = init_splat.to_array();
    let step = 0.2;

    let mut simplex: Vec<Vec<f32>> = vec![Vec::from(init_splat_arr)];
    for (i, init_val) in init_splat_arr.iter().enumerate() {
        let mut point = Vec::<f32>::from(init_splat_arr);
        point[i] += step;

        simplex.push(point);
    }

    let solver = NelderMead::new(simplex);

    let problem = OptimizeSplat {
        goal_img,
        curr_img
    };

    let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(n_trials))
        .run()
        .expect("Failed to run the Splat optimizer");

    let best_vals = result.state().best_param.as_ref().unwrap();
    let best_splat = Splat::from_indexable(best_vals);
    let best_error = result.state().best_cost;

    return (best_splat, best_error);
}