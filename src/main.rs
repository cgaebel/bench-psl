#![allow(unstable)]

use std::collections::{hash_map, HashMap, HashSet};
use std::default::Default;
use std::iter::range;
use std::num::Int;
use std::sync::Future;

fn real_cap_of(load_factor: f64, base_cap: usize) -> usize {
    let hm: HashMap<usize, ()> =
        HashMap::with_capacity_hash_state_and_load_factor(
            base_cap, Default::default(), load_factor);

    hm.capacity()
}

fn doit(load_factor: f64, cap: usize, seen_caps: &mut HashSet<usize>, summary: bool) {
    let real_cap = {
        let cap = real_cap_of(load_factor, cap);
        if seen_caps.contains(&cap) { return; }
        seen_caps.insert(cap);
        cap
    };

    let experiments = 1000;

    let mut tot_avg_psl = 0.;
    let mut tot_max_psl = 0.;

    for _ in range(0, experiments) {
        let mut hm: HashMap<usize, ()> =
            HashMap::with_capacity_hash_state_and_load_factor(
                cap, Default::default(), load_factor);

        for j in range(0, real_cap) {
            hm.insert(j, ());
        }

        let (avg_psl, max_psl) = hm.avg_max_psl();

        tot_avg_psl += avg_psl;
        tot_max_psl += max_psl;

        if !summary {
            println!("{},{},{},{}", load_factor, real_cap, avg_psl, max_psl);
        }
    }

    let avg_psl = tot_avg_psl / experiments as f64;
    let max_psl = tot_max_psl / experiments as f64;

    if summary {
        println!("{},{},{},{}", load_factor, real_cap, avg_psl, max_psl);
    }
}

/// Outputs data for avg+max psl for a fully loaded hash table with a variety of
/// load factors.
fn bench_max_capacity() {
    println!("load factor,capacity,avg_psl,max_psl");
    let summary = false;

    // test load factors
    for load_factor in 70..101 {
        let load_factor = load_factor as f64 / 100.0;
        let mut seen_caps = HashSet::new();
        for cap in 16..10000 {
            doit(load_factor, cap, &mut seen_caps, summary);
        }
    }
}

/// Outputs data for avg+max psl for a hash table growing from 0 elements to
/// 1,000,000.
fn bench_growth() {
    println!("load factor,grow to,avg psl,avg max psl");

    for load_factor in 70..100+1 {
        let load_factor = load_factor as f64 / 100.0;

        for grow_to_exp in 2..7+1 {
            let grow_to = 10u64.pow(grow_to_exp);

            // Try this a bunch of times.
            let num_trials = 8;

            let mut futs = vec!();

            for _ in range(0, num_trials) {
                futs.push(Future::spawn(move || {
                    let mut hm: HashMap<usize, ()> =
                        HashMap::with_capacity_hash_state_and_load_factor(
                            hash_map::INITIAL_CAPACITY, Default::default(), load_factor);

                    let mut tot_avg_psl = 0u64;
                    let mut tot_max_psl = 0;

                    for i in range(0, grow_to as usize) {
                        hm.insert(i, ());
                        let (my_avg_psl, my_max_psl) = hm.avg_max_psl();
                        tot_avg_psl += my_avg_psl as u64;
                        tot_max_psl += my_max_psl as u64;
                    }

                    let avg_psl = tot_avg_psl as f64 / grow_to as f64;
                    let avg_max_psl = tot_max_psl as f64 / grow_to as f64;

                    (avg_psl, avg_max_psl)

                }));
            }

            for f in futs.into_iter() {
                let (avg_psl, avg_max_psl) = f.into_inner();

                println!("{},{},{},{}", load_factor, grow_to, avg_psl, avg_max_psl);
            }
        }
    }
}

fn main() {
    if true {
        bench_max_capacity();
    } else {
        bench_growth();
    }
}
