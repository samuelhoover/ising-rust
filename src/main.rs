use full_palette::ORANGE_500;
use plotters::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const NCOLS: usize = 1_000;
const NROWS: usize = 1_000;
const LEN: usize = NCOLS * NROWS;
const STEPS: u32 = 1_000_000_000;

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
            let _ = area.fill(&BLUE);
        } else {
            let _ = area.fill(&ORANGE_500);
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

fn run(j: i8, beta: f32) -> () {
    let mut rng: SmallRng = SmallRng::from_entropy();

    // initialize lattice
    let mut arr: [i8; LEN] = initialize_lattice(&mut rng);

    // pre-calculate the probabilities to use for conditional acceptance
    // 9 probabilities because Delta_H can only be -8, -6, ..., 0, ..., 6, 8
    let mut probs = [0u32; 9];
    let mut increment: f32 = 4.0;
    for prob in &mut probs {
        let p = f32::exp(-2.0 * beta * increment);
        *prob = (2f32.powi(64) * p) as u32;
        increment -= 1.0;
    }

    plot(&arr, "t_0.png");

    let mut count_a: u32 = 0;
    let mut count_ca: u32 = 0;
    let mut count_r: u32 = 0;
    for _ in 0..STEPS {
        // randomly select lattice site
        let i: usize = rng.gen_range(0..LEN);

        // get neighbors
        let (up, down, left, right) = get_neigbors(&i);

        // calculate energy with and without flip
        let energy_old: i8 = -j * arr[i] * (arr[up] + arr[down] + arr[left] + arr[right]);
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

    println!("Accepted: {count_a}\nConditionally accepted: {count_ca}\nRejected: {count_r}");

    let name = format!("t_{STEPS}.png");
    plot(&arr, &name[..]);
}

fn main() {
    let j: i8 = 1;
    let beta: f32 = 10.0;

    run(j, beta);
}
