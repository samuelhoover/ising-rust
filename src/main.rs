use ising::ising;

fn main() {
    let beta: f32 = 10.0;
    let print: bool = false;
    ising::run(beta, print);
}
