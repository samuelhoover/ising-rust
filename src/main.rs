use ising::ising;

fn main() {
    let beta: f32 = 10.0;
    let print: bool = true;
    ising::run(beta, print);
}
