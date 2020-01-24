// binner
//  Command-line tool for reading in a list of numbers and writing 
//  bins and counts for a specified bin width 
//
//  bins are defined [lower, upper), such that each output point
//  has the form:
//  (x,y) = ((lower + upper) / 2, counts)
//  and upper - lower = specified binwidth

#[macro_use]
extern crate clap;

use std::io::{self, BufRead};
use std::cmp::Ordering;

use clap::{Arg, App}; // for parsing command-line arguments

fn bins(values: &Vec<f64>, binwidth: f64, binstart: f64) -> Vec<(f64, u32)> {
    // Input: a vector of floats and a bin width 
    // Output: a vector of bin edges and counts for each bin

    // unique integer identifier for each bin, to be used to check if bin is present
    // the actual bin edges will be computed as floats, so this avoids 
    // checking for strict float equality
    let mut bin_ids: Vec<i32> = Vec::new();

    // computed bin edge values and counts 
    let mut edges_counts: Vec<(f64, u32)> = Vec::new();

    for val in values {
        let bin_id = (val / binwidth).floor() as i32;

        // check if the bin is already in the vector
        match bin_ids.iter().position(|&b| b == bin_id) {
            // if it is, add to the corresponding counts
            Some(i) => { edges_counts[i].1 += 1 },
            // else, add a new bin
            _ => {
                bin_ids.push(bin_id);
                edges_counts.push(
                    (binwidth * (((val - binstart) / 
                      binwidth).floor() + 0.5) + binstart, 1)
                );
            },
        };
    }

    // sort result by edge, then return. Since floats may contain NaN,
    // we need to use partial_cmp since edges are floats,
    // and specify what to do for errors
    edges_counts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    return edges_counts;
}


fn main() {
    // parse command line arguments and set up help with clap-rs
    let arg_matches = App::new("binner")
        .version("1.0")
        .about("Read numbers from standard input, output bins and counts")
        .arg(Arg::with_name("binwidth")
             .short("w")
             .value_name("WIDTH")
             .help("Set bin width")
             .takes_value(true)
             .default_value("1.0"))
        .arg(Arg::with_name("binstart")
             .short("s")
             .value_name("EDGE START")
             .help("Set bin edge start")
             .takes_value(true)
             .default_value("0.0"))
        .arg(Arg::with_name("INPUT")
             .help("Input stream")
             .index(1))
        .get_matches();

    // type-checking macro explaind in clap example 12_typed_values.rs
    let binwidth: f64 = value_t!(arg_matches, "binwidth", f64).unwrap_or(1.0);
    let binstart: f64 = value_t!(arg_matches, "binstart", f64).unwrap_or(0.0);

    // parse stdin to make a list of values 
    let stdin = io::stdin();
    let mut values: Vec<f64> = Vec::new();
    for line in stdin.lock().lines() {
        let elem: f64 = match line.unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid value entered");
                continue;
            },
        };
        values.push(elem);
    }

    // compute bins and print
    let result = bins(&values, binwidth, binstart); 
    for bin in &result {
        println!("{}\t{}", bin.0, bin.1);
    }
}


#[cfg(test)]
mod tests{
    use super::bins;

    #[test]
    fn sequence() {
        // bin 10 numbers. bin edges start at binwdith / 2 intervals
        let vals = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5];
        let out = vec![(1.5, 2), (2.5, 2), (3.5, 2), (4.5, 2), (5.5, 2)];
        assert_eq!(bins(&vals, 1.0, 1.0), out); 
    }

    #[test]
    fn unordered() {
        let vals = vec![1.0, 55.6, -15.2, 55.9];
        let out = vec![(-15.5, 1), (1.5, 1), (55.5, 2)];
        assert_eq!(bins(&vals, 1.0, 1.0), out); 
    }
}
