use crate::consts::*;
use crate::utils::{get_neigbors, initialize_lattice, plot, precalculate_probs};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[inline]
pub fn run(beta: f32) -> () {
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
