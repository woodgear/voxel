use std::time::{Duration, SystemTime};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let now = SystemTime::now();

    let n = 80;
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                do_cube(x, y, z)?;
            }
        }
    }
    let elapsed = now.elapsed()?;
    let fps_60 = Duration::from_millis(16);

    println!(
        "Hello, world! {:?} ok {} per {:.5}% ",
        elapsed,
        elapsed < fps_60,
        percentage(elapsed, fps_60)
    );
    println!("{} {}", elapsed.as_nanos(), fps_60.as_nanos());
    return Ok(());
}

fn do_cube(x: u32, y: u32, z: u32) -> Result<(), anyhow::Error> {
    return Ok(());
}
fn percentage(left: Duration, right: Duration) -> f64 {
    let l_ns = left.as_nanos();
    let r_ns = right.as_nanos();
    return floatconv::fast::u128_to_f64_round(l_ns) / floatconv::fast::u128_to_f64_round(r_ns)
        * 100.0;
}
