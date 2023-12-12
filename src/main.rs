#![allow(unused_parens)]

mod cli;
mod tsp_lib;
mod blackbird;
mod util;

use clap::Parser;

use cli::CliArgs;
use crate::blackbird::data::bb_data::BBData;

// Blackbird:
// B Bringing to you a derivation of the
// L Lin Kernighan Heuristic by
// A Analyizing 
// C Concorde,
// K Keen on
// B Building a somewhat
// I Identically performing,
// R Rust-written
// D Duplicate.

// DESIGN DECISIONS
// - Focus on 2 dimensions, perhaps 3 dimensions if time allows it
//
// - Main focus on euclidean distance, but trying to make extensions for other
//   metrics possible
//
// - Using available kd implementation
//
// - No read in of initial cycle - Goal is to get a "good" solution without 
//   prior knowledge
//
// - No read in of sparse edge set - instead this gets generated (kd-tree stuff)
//
// - Usage of ready KD-Tree implementation - this alone break "compatibility"
//   with Concorde, as KD-Tree generation is not deterministic, 
//
// - See linkern.c.old regarding (de)activated macros:
//   Old version of linkern.c that contains all of the ifdef macros for
//   USE_HEAP, FULL_MAK_MORTON, ACCEPT_BAD_TOURS, SUBTRACT_GSTAR, SWITCH_LATE, 
//   NODE_INSERTIONS, USE_LESS_OR_EQUAL, USE_LESS_MARKING, BENTLEY_CACHE,
//   LONG_KICKER and MARK_NEIGHBORS
//   Some of them are now activated in linkern.c, some are deactivated, 
//   depending on the default configuration values.
//
// - The USE_SPACEFILL macro in kdspan is assumed to be NOT defined. This is
//   relevant for the quick boruvka tour generation
// - The sorting in the look_ahead / lk_ordering functions have been reversed
//   when compared to Concorde. During development, this sorting yielded better
//   results

// Example run via:
// cargo run -- -i ../../tsp_examples/burma14.tsp

fn 
main() 
{
	// Get the CLI arguments
	let args = CliArgs::parse();

	// Print the seed used for this run for the shell script that calls Blackbird
	println!("Seed : {}", args.random_generator_seed);

	// Create the blackbird data structure
	let mut bb_data = BBData::new(args);

	// Output the number of nodes for the shell script that calls Blackbird
	println!("Number of nodes : {}", bb_data.tsp_data.n);

	// Apply the CLK heuristic
	bb_data.main_heuristic();
}
