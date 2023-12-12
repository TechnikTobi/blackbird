use rand::seq::SliceRandom;

use crate::blackbird::data::bb_edge::BBEdge;
use crate::blackbird::data::bb_tour::BBTour;

use crate::blackbird::data::bb_data::BBData;
use crate::tsp_lib::node::TSPNodeID;

impl
BBData
{
	pub fn
	create_initial_tour_random
	(
		&mut self
	)
	{
		// Collect the ids of all nodes
		let mut local_node_ids = self.tsp_data.nodes.iter().map(|node| node.id).collect::<Vec<TSPNodeID>>();

		// Shuffle the ids
		local_node_ids.shuffle(&mut self.random_generator);

		// Initialize the new tour
		let mut new_tour = BBTour::new();

		// Construct the edges
		for i in 0..local_node_ids.len()-1
		{
			// Add the new edge to the tour
			new_tour.add(BBEdge::new(&self.tsp_data, local_node_ids[i], local_node_ids[i+1]));
		}

		// Close the tour
		new_tour.add(BBEdge::new(&self.tsp_data, local_node_ids[0], local_node_ids[local_node_ids.len()-1]));

		// Replace current with new tour
		self.current_tour = Some(new_tour);
	}
}
