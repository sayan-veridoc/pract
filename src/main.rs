use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    for _ in 1..=1_000_000_000 {}

    let elapsed_time = start_time.elapsed();

    println!("Time taken: {:.2?}", elapsed_time);
}
