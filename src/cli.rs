use clap::Parser;

use crate::tsp_lib::node::TSPWeight;

#[derive(Clone, Parser, Debug)]
#[command(version)]
#[command(about, long_about = None)]
/// Blackbird - Reverse Engineering CONCORDE CLK by Tobias Prisching, B. Sc. (11911362)
pub struct 
CliArgs
{
	/// The path to the .tsp input file
	#[arg(short='i', long, required=false, default_value="")]
	pub node_input_file_path: String,

	/// How to create the initial tour ('r': Random; 'b': Boruvka; 'q': Quick Boruvka [default])
	#[arg(short='c', long, required=false, default_value="q")]
	pub init_tour_method: char,

	/// Verbose output to terminal
	#[arg(short='v', long, required=false)]
	pub verbose: bool,

	/// A time limit for when to stop applying the heurisitic to guarantee a given runtime
	#[arg(short='t', long, required=false, default_value="1000000")]
	pub time_limit: u64,

	/// A tour length limit for when to stop applying the heuristic once a certain length is reached
	#[arg(short='l', long, required=false, default_value="0.0")]
	pub length_limit: TSPWeight,

	/// Seed for the random generator (where applicable). If seed is 0, use current time as seed.
	#[arg(short='s', long, required=false, default_value="0")]
	pub random_generator_seed: u64,

	/// The number of times to apply the CLK heuristic
	#[arg(short='r', long, required=false, default_value="0")]
	pub number_of_runs: u64,

	/// Use quadrant #-nearest for the sparse edge set
	#[arg(short='q', long, required=false, default_value="2")]
	pub quadrant_nearest_count: usize,
}
