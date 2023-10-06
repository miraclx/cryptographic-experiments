use std::time;

// const DEFAULT_SAMPLES: usize = 1000;

macro_rules! bench {
    // ($( $name:literal: $fn:expr ),+) => {
    //     bench!(SAMPLES: $crate::bench::DEFAULT_SAMPLES, $( $name: $fn ),+);
    // };
    (samples: $samples: expr, $( $name:literal: $fn:expr ),+) => {
        {
            {
                let samples = $samples;
                let mut bench_results = vec![];
                $(
                    bench_results.push($crate::bench::bench_one($name, samples, $fn));
                )+
                $crate::bench::bench_report(bench_results);
            };
        }
    };
}

pub(crate) use bench as run;

pub fn bench_report(mut results: Vec<(&str, time::Duration)>) {
    results.sort_by_key(|(_, dur)| *dur);
    if results.len() > 1 {
        println!("\x1b[1mSummary\x1b[0m");
        let mut bench_results = results.into_iter();
        let fastest = bench_results.next().unwrap();
        println!("  \x1b[36m{}\x1b[0m ran", fastest.0);
        for (name, dur) in bench_results {
            println!(
                "    \x1b[1;32m{:.2}\x1b[0m times faster than \x1b[35m{}\x1b[0m",
                dur.as_secs_f64() / fastest.1.as_secs_f64(),
                name
            );
        }
    }
}

pub fn bench_one<F>(name: &str, samples: usize, f: F) -> (&str, time::Duration)
where
    F: Fn(),
{
    println!("\x1b[1mbench:\x1b[0m {}", name);
    let mut records = vec![];
    for _ in 0..samples {
        let start = time::Instant::now();
        f();
        records.push(start.elapsed());
    }
    let (min, max, total) = records.iter().fold(
        (
            time::Duration::MAX,
            time::Duration::ZERO,
            time::Duration::ZERO,
        ),
        |(min, max, total), dur| {
            let min = if dur < &min { *dur } else { min };
            let max = if dur > &max { *dur } else { max };
            (min, max, total + *dur)
        },
    );
    let mean = total / samples as u32;
    println!(
        "  Time (\x1b[1;32mmean\x1b[0m): \x1b[1;32m{: >18.2?}\x1b[0m",
        mean
    );
    println!(
        "  Range (\x1b[36mmin\x1b[0m … \x1b[35mmax\x1b[0m):\x1b[36m{: >13.2?}\x1b[0m … \x1b[35m{: <8.2?}\x1b[0m     [{} samples]",
        min, max, samples
    );
    println!("  Total: \x1b[1;36m{:.2?}\x1b[0m", total);
    (name, mean)
}
