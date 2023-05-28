use rand::Rng;

fn main() {
    let mut inside_circle = 0u64;
    let mut inside_circle_stratified = 0u64;
    let mut rng = rand::thread_rng();

    const SQRT_N: u64 = 10000;

    for i in 0..SQRT_N {
        for j in 0..SQRT_N {
            let mut x: f64 = rng.gen_range(-1.0..=1.0);
            let mut y: f64 = rng.gen_range(-1.0..=1.0);

            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            x = (2.0f64 * (i as f64 + rng.gen::<f64>()) / SQRT_N as f64) - 1.0;
            y = (2.0f64 * (j as f64 + rng.gen::<f64>()) / SQRT_N as f64) - 1.0;

            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    let n = SQRT_N as f64 * SQRT_N as f64;

    println!(
        "regular estimation = {} stratified estimate = {}",
        4.0 * inside_circle as f64 / n,
        4.0 * inside_circle_stratified as f64 / n
    );
}
