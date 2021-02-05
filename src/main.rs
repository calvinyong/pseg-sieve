mod primality;
mod sieve;

use clap::{App, AppSettings, Arg, SubCommand};
use std::time::Instant;

fn bench_algs() {
    let segments = 20000;
    let limit = 1_000_000;

    println!("Benching for primes up to {}\n", limit);

    let now = Instant::now();
    let _ = primality::sequential_primality(limit);
    let elapsed = now.elapsed().as_secs_f32();
    println!("Sequential primality: {}", elapsed);

    let now = Instant::now();
    primality::rayon_primality(limit);
    let elapsed = now.elapsed().as_secs_f32();
    println!("Parallel primality: {}", elapsed);

    let now = Instant::now();
    let _ = sieve::sequential_sieve(limit);
    let elapsed = now.elapsed().as_secs_f32();
    println!("Sieve of Eratosthenes: {}", elapsed);

    let now = Instant::now();
    let _ = sieve::sequential_segmented_sieve(limit, segments);
    let elapsed = now.elapsed().as_secs_f32();
    println!("Sequential segmented sieve: {}", elapsed);

    let now = Instant::now();
    let _ = sieve::rayon_segmented_sieve(limit, segments);
    let elapsed = now.elapsed().as_secs_f32();
    println!("Parallel segmented sieve: {}", elapsed);
}

fn bench_seg_sieve() {
    let limit = 1_000_000_000;
    let segments: [usize; 6] = [1000, 5000, 10000, 20000, 30000, 50000];

    println!("alg,segments,time");
    for &segment in segments.iter() {
        let now = Instant::now();
        let _ = sieve::sequential_segmented_sieve(limit, segment);
        let elapsed = now.elapsed().as_secs_f32();
        println!("Sequential,{},{}", segment, elapsed);

        let now = Instant::now();
        let _ = sieve::rayon_segmented_sieve(limit, segment);
        let elapsed = now.elapsed().as_secs_f32();
        println!("Parallel,{},{}", segment, elapsed);
    }
}

fn main() {
    let matches = App::new("pseg-sieve")
        .author("Calvin")
        .about("Parallel segmented sieve with benchmarks")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the parallel segmented sieve")
                .arg(
                    Arg::with_name("limit")
                        .short("l")
                        .takes_value(true)
                        .default_value("1000000000")
                        .help("Find primes up to limit"),
                )
                .arg(
                    Arg::with_name("segments")
                        .short("n")
                        .takes_value(true)
                        .default_value("20000")
                        .help("Number of segments"),
                ),
        )
        .subcommand(
            SubCommand::with_name("bench-algs").about("Benchmark primality algs and sieves"),
        )
        .subcommand(
            SubCommand::with_name("bench-seg-sieve")
                .about("Benchmark segmented sieve varying number of segments"),
        )
        .get_matches();

    if let Some(run) = matches.subcommand_matches("run") {
        let limit: usize = run
            .value_of("limit")
            .unwrap()
            .parse()
            .expect("Failed to parse limit");

        let n_segments: usize = run
            .value_of("segments")
            .unwrap()
            .parse()
            .expect("Failed to parse segments");

        println!(
            "Timing parallel segmented sieve with {} segments for primes up to {}",
            n_segments, limit
        );

        let now = Instant::now();
        let _ = sieve::rayon_segmented_sieve(limit, n_segments);
        let elapsed = now.elapsed().as_secs_f32();

        println!("Parallel segmented sieve took {} seconds", elapsed);
    } else if let Some(_) = matches.subcommand_matches("bench-algs") {
        bench_algs();
    } else if let Some(_) = matches.subcommand_matches("bench-seg-sieve") {
        bench_seg_sieve();
    }
}
