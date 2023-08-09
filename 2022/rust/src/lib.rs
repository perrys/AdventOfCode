use std::time::Instant;

#[inline(always)]
pub fn timer<F: FnMut()>(mut func: F) {
    const N_ITERS: u32 = 10000;
    let mut sum: u128 = 0;
    let mut sumsq: u128 = 0;
    let mut opt_min_dur: Option<u128> = None;
    for _ in 0..N_ITERS {
        let now = Instant::now();
        func();
        let duration = now.elapsed().as_nanos();
        sum += duration;
        sumsq += duration * duration;
        match opt_min_dur {
            Some(ref mut min_dur) => {
                if duration < *min_dur {
                    *min_dur = duration;
                }
            }
            None => opt_min_dur = Some(duration),
        }
    }
    let avg = sum as f64 / (N_ITERS as f64);
    let stddev = ((sumsq as f64 / (N_ITERS as f64)) - (avg * avg)).sqrt();
    let min_time = opt_min_dur.unwrap();
    println!("Elapsed time - min: {min_time:}ns, avg: {avg:.0}, stddev: {stddev:.0}");
}
