use plotters::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const NCOLS: usize = 2_000;
const NROWS: usize = 2_000;
const LEN: usize = NCOLS * NROWS;
const STEPS: u64 = 1_000_000_000;
const J: i8 = 1;

fn initialize_lattice(rng: &mut SmallRng) -> [i8; LEN] {
    let mut arr: [i8; LEN] = [0i8; LEN];
    rng.fill(&mut arr[..]);
    for site in arr.iter_mut() {
        *site = if rng.gen_bool(0.5) { 1i8 } else { -1i8 }
    }

    arr
}

fn plot(&arr: &[i8; LEN], name: &str) {
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

fn get_neigbors(&i: &usize) -> (usize, usize, usize, usize) {
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

fn precalculate_probs(beta: f32) -> [u32; 9] {
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

fn run(beta: f32) -> () {
    let mut rng: SmallRng = SmallRng::from_entropy();

    // initialize lattice
    let mut arr: [i8; LEN] = initialize_lattice(&mut rng);

    // precalculate probabilities for conditional acceptance
    let probs: [u32; 9] = precalculate_probs(beta);

    plot(&arr, "t_0.png");

    let mut count_a: u64 = 0;
    let mut count_ca: u64 = 0;
    let mut count_r: u64 = 0;
    for _ in 0..STEPS {
        // randomly select lattice site
        let i: usize = rng.gen_range(0..LEN);

        // get neighbors
        let (up, down, left, right) = get_neigbors(&i);

        // calculate energy with and without flip
        let energy_old: i8 = -J * arr[i] * (arr[up] + arr[down] + arr[left] + arr[right]);
        // energy with flip is the negative of `energy_old`

        // perform/reject flip
        if -energy_old < 0 {
            arr[i] *= -1;
            count_a += 1;
        } else if rng.gen::<u32>() < probs[(4 + energy_old) as usize] {
            arr[i] *= -1;
            count_ca += 1;
        } else {
            count_r += 1;
        }
    }

    // Print Monte Carlo simulation stats
    let a: String = format!(
        "Accepted:               {count_a} [{:.1}%]",
        100.0 * count_a as f32 / STEPS as f32
    );
    let b: String = format!(
        "Conditionally accepted: {count_ca} [{:.1}%]",
        100.0 * count_ca as f32 / STEPS as f32
    );
    let c: String = format!(
        "Rejected:               {count_r} [{:.1}%]",
        100.0 * count_r as f32 / STEPS as f32
    );
    let out: String = [a, b, c].join("\n");
    println!("{out}");

    // Save final result
    let suffix: u32 = STEPS.ilog10();
    let prefix: u64 = STEPS / 10u64.pow(suffix);
    let name: String = format!("t_{prefix}e{suffix}.png");
    plot(&arr, &name[..]);
}

fn main() {
    let beta: f32 = 10.0;

    run(beta);
}
