#![allow(unstable)]

use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::iter::range;

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

fn main() {
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
