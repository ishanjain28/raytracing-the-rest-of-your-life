use rand::Rng;

#[inline]
fn pdf(x: f64) -> f64 {
    3.0 * x * x / 8.0
}

fn main() {
    let mut rng = rand::thread_rng();
    const N: u64 = 100000;
    let mut sum = 0.0;

    for _ in 0..N {
        let x: f64 = rng.gen_range(0.0f64..=8.0).powf(1.0 / 3.0);

        sum += x * x / pdf(x);
    }

    println!("answer = {}", sum / N as f64);
}
