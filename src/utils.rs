use crate::consts::*;
use plotters::prelude::*;
use rand::rngs::SmallRng;
use rand::Rng;

pub fn initialize_lattice(rng: &mut SmallRng) -> [i8; LEN] {
    let mut arr: [i8; LEN] = [0i8; LEN];
    rng.fill(&mut arr[..]);
    for site in arr.iter_mut() {
        *site = if rng.gen_bool(0.5) { 1i8 } else { -1i8 }
    }

    arr
}

pub fn plot(&arr: &[i8; LEN], name: &str) {
    let root = BitMapBackend::new(name, (NCOLS as u32, NROWS as u32)).into_drawing_area();

    let areas = root.split_evenly((NCOLS, NROWS));

    for (area, i) in areas.into_iter().zip(0..LEN) {
        if arr[i] == 1i8 {
            let _ = area.fill(&RGBColor(216, 179, 101));
        } else {
            let _ = area.fill(&RGBColor(90, 180, 172));
        }
    }
}

pub fn get_neigbors(&i: &usize) -> (usize, usize, usize, usize) {
    // Get neighbors using periodic boundary conditions (PBCs) if necessary
    //
    // `if` statements contains neighbors when applying PBC, `else` statements contain actual
    // neighbors
    let up: usize = if i <= (NCOLS - 1) {
        i + LEN - NCOLS
    } else {
        i - NCOLS
    };
    let down: usize = if i >= LEN - NCOLS - 1 { i } else { i + NCOLS };
    let left: usize = if i % NCOLS == 0 { i + NCOLS - 1 } else { i - 1 };
    let right: usize = if (i + 1) % NCOLS == 0 {
        i + 1 - NCOLS
    } else {
        i + 1
    };

    (up, down, left, right)
}

pub fn precalculate_probs(beta: f32) -> [u32; 9] {
    // 9 probabilities because Delta_H can only be -8, -6, ..., 0, ..., 6, 8
    let mut probs: [u32; 9] = [0u32; 9];
    let mut increment: f32 = 4.0;
    for prob in &mut probs {
        let p: f32 = f32::exp(-2.0 * beta * increment);
        *prob = (2f32.powi(64) * p) as u32;
        increment -= 1.0;
    }

    probs
}