use std::collections::HashMap;
use std::time::Instant;

use crate::blackbird::data::bb_edge::BBEdge;
use crate::blackbird::data::bb_tour::BBTour;
use crate::blackbird::data::bb_data::BBData;

use crate::tsp_lib::node::*;

impl
BBData
{
	pub fn
	create_initial_tour_boruvka
	(
		&mut self
	)
	{
		let time = Instant::now();

		// Calling this is "safe" as the tree only gets constructed if it
		// doesn't exist yet
		self.construct_kd_tree();

		println!("Time for constructing KD tree: {}µs", time.elapsed().as_micros());

		// Defining some helper structures:

		// A "queue" of nodes that need to be processed and a map that stores
		// the distance to their nearest (viable) neighbors
		let mut nodes_to_be_processed = Vec::<&TSPNode>::new();
		let mut nearest_neighbors = HashMap::<TSPNodeID, (TSPWeight, Option<TSPNode>)>::new();

		// Store the degree of each node
		let mut degree = HashMap::<TSPNodeID, u8>::new();

		// Initialize the helper data structures
		for node in &self.tsp_data.nodes
		{
			nodes_to_be_processed.push(node);
			nearest_neighbors.insert(node.id, (TSPWeight::MAX, None));
			degree.insert(node.id, 0);
		}

		// Store the tails of each node
		let mut tails = HashMap::<TSPNodeID, TSPNodeID>::new();

		// Make local copy of the KD tree as it gets edited
		let mut local_kd_tree = self.kd_tree.as_mut().unwrap().clone();

		// The new tour that results from this function
		let mut new_tour = BBTour::new();

		while new_tour.edges.len() + 1 < self.tsp_data.n
		{
			let mut nodes_ids_that_do_not_need_further_processing = Vec::new();

			// Prior to the edge finding, get the current remaining nodes to be
			// processed, find the distance to their nearest neighbor and sort
			// them with ascending distance value
			for node in &nodes_to_be_processed
			{
				if degree[&node.id] < 2
				{
					let nearest_neighbor_with_distance;

					if tails.contains_key(&node.id)
					{
						// Disable the tail as it can't be a candidate due to
						// the edge to it already existing (otherwise it would
						// not be the tail of node)
						let tail_node = self.tsp_data.get_node(tails[&node.id]);
						local_kd_tree.disable_node(tail_node);

						let nearest_neighbor_candidates = local_kd_tree.nearests(node, 1, &self.tsp_data);

						// If no such node exists, we are done an only have one
						// final edge left to add to close the tour
						if nearest_neighbor_candidates.first().is_none()
						{
							break;
						}

						// Unpack the nearest neighbor an re-enable the tail as
						// it still needs a second edge
						nearest_neighbor_with_distance = nearest_neighbor_candidates.first().unwrap().clone();
						local_kd_tree.enable_node(tail_node);
					}
					else
					{
						nearest_neighbor_with_distance = local_kd_tree.nearests(node, 1, &self.tsp_data).first().unwrap().clone();
					}

					nearest_neighbors.insert(node.id,
						(
							nearest_neighbor_with_distance.1,
							Some(nearest_neighbor_with_distance.0.clone())
						)
					);

				}
				else
				{
					// This node already is of degree two and thus does not
					// require any further processing. Therefore remove it from
					// the list of nodes that need to be processed.
					nodes_ids_that_do_not_need_further_processing.push(node.id);
				}
			}

			// Remove all the nodes from the to-be-processed list that have
			// been deemed done being processed by the previous loop
			nodes_to_be_processed.retain(|remaining_node| !nodes_ids_that_do_not_need_further_processing.contains(&remaining_node.id));

			// Get the indices that would sort the nodes based on their first coordinate
			let mut sorting_indices = nodes_to_be_processed.iter().map(|node| node.id).collect::<Vec<_>>();
			sorting_indices.sort_by(
				|&a, &b| 
				nearest_neighbors[&a].0.partial_cmp(&nearest_neighbors[&b].0).unwrap()
			);

			for node_id in &sorting_indices
			{
				// Get the node that we want to look at
				let node = self.tsp_data.get_node(*node_id);

				// Check if the degree of the node makes sense to look at it
				// (if it already has degree 2 then we can't add an edge for it)
				if degree[&node.id] < 2
				{
					let nearest_neighbor = nearest_neighbors[&node.id].1.unwrap().clone();

					if degree[&nearest_neighbor.id] != 2 // && tails[&node.id] != nearest_neighbor.id
					{
						if tails.contains_key(&node.id)
						{
							if tails[&node.id] == nearest_neighbor.id
							{
								continue;
							}
						}

						// Remove nodes from the KD tree if they already have a
						// degree greater than 0
						if degree.get(&node.id).unwrap() > &0
						{
							local_kd_tree.disable_node(&node);
						}

						if degree.get(&nearest_neighbor.id).unwrap() > &0
						{
							local_kd_tree.disable_node(&nearest_neighbor);
						}

						// Increase degrees
						degree.insert(node.id,             degree[&node.id            ] + 1);
						degree.insert(nearest_neighbor.id, degree[&nearest_neighbor.id] + 1);

						// Insert edge
						let new_edge = BBEdge::new(&self.tsp_data, node.id, nearest_neighbor.id);
						new_tour.add(new_edge);

						// Handle the tails
						if !tails.contains_key(&node.id)
						{
							if !tails.contains_key(&nearest_neighbor.id)
							{
								// Both don't have a tail yet
								tails.insert(node.id, nearest_neighbor.id);
								tails.insert(nearest_neighbor.id, node.id);
							}
							else
							{
								// node does not have a tail yet but its partner does
								tails.insert(node.id, *tails.get(&nearest_neighbor.id).unwrap());
								tails.insert(*tails.get(&nearest_neighbor.id).unwrap(), node.id);
							}
						}
						else if !tails.contains_key(&nearest_neighbor.id)
						{
							// node does have a tail but its partner hasn't yet
							tails.insert(*tails.get(&node.id).unwrap(), nearest_neighbor.id);
							tails.insert(nearest_neighbor.id, *tails.get(&node.id).unwrap());
						}
						else
						{
							// Both already have tails
							tails.insert(*tails.get(&node.id).unwrap(), *tails.get(&nearest_neighbor.id).unwrap());
							tails.insert(*tails.get(&nearest_neighbor.id).unwrap(), *tails.get(&node.id).unwrap());
						}
					}
				}
			}
		}

		// Find the two nodes that haven't been connected yet via an edge
		let lonely_nodes = self.tsp_data.nodes.iter()
			.filter(|node| degree[&node.id] < 2)
			.collect::<Vec<&TSPNode>>();
		
		// There need to be *exactly* two such nodes
		assert!(lonely_nodes.len() == 2);

		// Add the final edge to complete the tour
		let new_edge = BBEdge::new(&self.tsp_data, lonely_nodes[0].id, lonely_nodes[1].id);
		new_tour.add(new_edge);

		// Place new tour in BBData struct
		self.current_tour = Some(new_tour);
		
	}

}
