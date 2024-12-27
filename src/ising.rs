use crate::consts::*;
use crate::utils::{
    get_neigbors, initialize_lattice, plot_count, plot_lattice, precalculate_probs,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[inline]
pub fn run(beta: f32, print: bool) -> () {
    // define pseudorandom number generator
    let mut rng: SmallRng = SmallRng::from_entropy();

    // initialize lattice
    let mut arr: [i8; LEN] = initialize_lattice(&mut rng);

    // precalculate probabilities for conditional acceptance
    let probs: [u32; 9] = precalculate_probs(beta);

    // plot initial lattice
    plot_lattice(&arr, "t_0.png");

    //// initialize counters
    // relative spins (0 -> equal, >0 -> more positive, <0 -> more negative)
    let mut count = vec![0i32; (STEPS + 1) as usize];
    count[0] = arr.iter().fold(0i32, |sum, i| sum + (*i as i32));
    // accepted flips
    let mut count_a: u64 = 0;
    // conditionally accepted flips
    let mut count_ca: u64 = 0;
    // rejected flips
    let mut count_r: u64 = 0;

    for i in 1..=STEPS {
        // randomly select lattice site
        let site: usize = rng.gen_range(0..LEN);
        assert!(site < LEN, "Site index out of range!");

        // get neighbors
        let (up, down, left, right) = get_neigbors(&site);

        // calculate energy before proposed flip
        let energy_old: i8 = -J * arr[site] * (arr[up] + arr[down] + arr[left] + arr[right]);

        // accept/reject flip
        if -energy_old < 0 {
            count[i as usize] = count[(i as usize) - 1] + (-2 * arr[site]) as i32;
            count_a += 1;
            arr[site] *= -1;
        } else if rng.gen::<u32>() < probs[(4 + energy_old) as usize] {
            count[i as usize] = count[(i as usize) - 1] + (-2 * arr[site]) as i32;
            count_ca += 1;
            arr[site] *= -1;
        } else {
            count[i as usize] = count[(i as usize) - 1];
            count_r += 1;
        }
    }

    // double check spin count
    let count_check: i32 = arr.iter().filter(|&n| *n == 1).count() as i32
        - arr.iter().filter(|&n| *n == -1).count() as i32;
    assert!(count[STEPS as usize] == count_check, "Count check failed!");

    // print Monte Carlo simulation stats
    if print {
        let str_count: String = format!("\nRelative count: {}\n", count[STEPS as usize]);
        let a: String = format!(
            "Accepted:               {count_a} [{:.1}%]",
            100.0 * count_a as f32 / STEPS as f32
        );
        let b: String = format!(
            "Conditionally accepted: {count_ca} [{:.1}%]",
            100.0 * count_ca as f32 / STEPS as f32
        );
        let c: String = format!(
            "Rejected:               {count_r} [{:.1}%]\n",
            100.0 * count_r as f32 / STEPS as f32
        );
        let out: String = [str_count, a, b, c].join("\n");
        println!("{out}");
    }

    // save final result
    let suffix: u32 = STEPS.ilog10();
    let prefix: u64 = STEPS / 10u64.pow(suffix);
    let name: String = format!("t_{prefix}e{suffix}.png");
    plot_lattice(&arr, &name[..]);

    // plot time evolution of relative spin count
    let _ = plot_count(count, "count.png");
}
