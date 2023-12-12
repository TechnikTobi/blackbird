use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use rand::rngs::StdRng;

use crate::blackbird::kd::kd_tree::KDtree;
use crate::cli::*;

use crate::tsp_lib::node::*;
use crate::tsp_lib::data::*;
use crate::tsp_lib::reader::*;

use crate::blackbird::data::bb_edge::*;
use crate::blackbird::data::bb_tour::*;
use crate::blackbird::initial_tour::method::EInitialTourMethod;

use super::bb_flipper::BBFlipper;

/// The BBData struct (BB = BlackBird) is the superset of all the data needed
/// for the TSP computation. This includes the "raw" TSPData as read from file
/// but also all the other information that gets computed from this input data
/// or is provided in some other way (e.g. the set of good edges to use during
/// the application of the heuristic)
pub struct 
BBData
{
	pub tsp_data:                      TSPData,
	pub sparse_edge_map:               HashMap<TSPNodeID, Vec<TSPNodeID>>,
	pub neighbour_candidates:          HashMap<TSPNodeID, Vec<TSPNodeID>>,
	pub cli_args:                      CliArgs,
	pub current_tour_length:           TSPWeight,
	pub init_method:                   EInitialTourMethod,
	pub kd_tree:                       Option<KDtree>,

	pub node_queue:                    VecDeque<TSPNodeID>,
	pub edge_markings:                 HashMap<BBEdge, EEdgeMarking>,

	pub current_tour:                  Option<BBTour>,
	pub initial_tour_cycle:            Option<Vec<TSPNodeID>>,
	pub flipper:                       Option<BBFlipper>,
	pub output_tour_cycle:             Option<Vec<TSPNodeID>>,
	pub output_tour_length:            TSPWeight,

	// Needed for the alternate_step 
	pub weirdmark:                     HashMap<TSPNodeID, i64>,
	pub weirdmagic:                    i64,

	pub random_generator:              StdRng,
}

impl
BBData
{
	pub fn
	new
	(
		cli_args: CliArgs
	)
	-> BBData
	{
		// Create a random generator, based either on the given seed or the
		// current UNIX time in seconds
		let random_generator = rand_seeder::Seeder::from(
			if cli_args.random_generator_seed > 0 
			{
				cli_args.random_generator_seed
			}
			else
			{
				SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.expect("Time went backwards")
					.as_secs()
			}
		).make_rng();

		BBData
		{
			tsp_data:                  read_tsplib_file(&cli_args.node_input_file_path),
			sparse_edge_map:           HashMap::new(),
			neighbour_candidates:      HashMap::new(),
			cli_args:                  cli_args.clone(),
			current_tour_length:       TSPWeight::MAX,
			init_method:               EInitialTourMethod::from_str(cli_args.init_tour_method.to_string().as_str()).unwrap(),
			kd_tree:                   None,

			node_queue:                VecDeque::new(),
			edge_markings:             HashMap::new(),

			current_tour:              None,
			initial_tour_cycle:        None,
			flipper:                   None,
			output_tour_cycle:         None,
			output_tour_length:        TSPWeight::MAX,

			weirdmark:                 HashMap::new(),
			weirdmagic:                0,

			random_generator:          random_generator,
		}
	}

	/// Marks an edge as being deleted during tour improvement 
	/// These markings are required for the creations of the orderings used in
	/// the step methods
	pub fn
	mark_edge_as_deleted
	(
		&mut self,
		edge:                          &BBEdge
	)
	{
		self.edge_markings.insert(edge.clone(), EEdgeMarking::DELETED);
	}

	/// Removes the deleted marking from an edge
	pub fn
	unmark_edge_as_deleted
	(
		&mut self,
		edge:                          &BBEdge
	)
	{
		self.edge_markings.insert(edge.clone(), EEdgeMarking::NONE);
	}

	/// Checks if a given edge has the deleted marking
	pub fn
	is_edge_deleted
	(
		&self,
		edge:                          &BBEdge
	)
	-> bool
	{
		if let Some(marking) = self.edge_markings.get(edge)
		{
			return marking.eq(&EEdgeMarking::DELETED);
		}
		return false;
	}

	/// Marks an edge as being added during tour improvement 
	/// These markings are required for the creations of the orderings used in
	/// the step methods
	pub fn
	mark_edge_as_added
	(
		&mut self,
		edge:                          &BBEdge
	)
	{
		self.edge_markings.insert(edge.clone(), EEdgeMarking::ADDED);
	}

	/// Removes the added marking from an edge
	pub fn
	unmark_edge_as_added
	(
		&mut self,
		edge:                          &BBEdge
	)
	{
		self.edge_markings.insert(edge.clone(), EEdgeMarking::NONE);
	}

		/// Checks if a given edge has the added marking
	pub fn
	is_edge_added
	(
		&self,
		edge:                          &BBEdge
	)
	-> bool
	{
		if let Some(marking) = self.edge_markings.get(edge)
		{
			return marking.eq(&EEdgeMarking::ADDED);
		}
		return false;
	}
}