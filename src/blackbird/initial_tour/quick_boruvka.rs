use crate::blackbird::data::bb_data::BBData;
use crate::blackbird::data::bb_edge::BBEdge;
use crate::blackbird::data::bb_tour::BBTour;
use crate::tsp_lib::node::*;

impl
BBData
{
	/// Implements the Quick-Boruvka tour creation algorithm as described in
	/// "The Traveling Salesman Problem: A Computational Study" by
	/// - David L. Applegate
	/// - Robert E. Bixby
	/// - Vašek Chvatál
	/// - William J. Cook
	/// ISBN: 9780691129938
	/// Page 455f
	/// In CONCORDE, this can be found as 
	/// "CCkdtree_qboruvka_tour" in kdspan.c
	pub fn
	create_initial_tour_quick_boruvka
	(
		&mut self
	)
	{
		// Calling this is "safe" as the tree only gets constructed if it
		// doesn't exist yet
		self.construct_kd_tree();

		// Get the indices that would sort the nodes based on their first coordinate
		let mut sorting_indices = (0..self.tsp_data.n).collect::<Vec<_>>();
		sorting_indices.sort_by(
			|&a, &b| 
			self.tsp_data.nodes[a].x.partial_cmp(&self.tsp_data.nodes[b].x).unwrap()
		);

		// Defining some helper structures:

		// Store the degree of each node
		let mut degree = vec![0; self.tsp_data.n];

		// Store the tails of each node
		let mut tails = vec![TSPNodeID::MAX; self.tsp_data.n];

		// The new tour that results from this function
		let mut new_tour = BBTour::new();

		// Make local copy of the KD tree as it gets edited
		let mut local_kd_tree = self.kd_tree.as_mut().unwrap().clone();

		while new_tour.edges.len() + 1 < self.tsp_data.n
		{
			for index in &sorting_indices
			{
				// Get the node that we want to look at
				// Remember: This way, they are traversed by ascending x coordinate
				let node = &self.tsp_data.nodes[*index];

				// Check if the degree of the node makes sense to look at it
				// (if it already has degree 2 then we can't add an edge for it)
				if degree[node.id] < 2
				{
					// The other node to construct an edge with
					// This is within the kNN neighborhood of 'node' but must not
					// be either the node itself or its current tail
					let nearest_neighbor;

					if tails[node.id] < TSPNodeID::MAX
					{
						// Disable the tail as it can't be a candidate due to
						// the edge to it already existing (otherwise it would
						// not be the tail of node)
						let tail_node = self.tsp_data.get_node(tails[node.id]);
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
						nearest_neighbor = nearest_neighbor_candidates.first().unwrap().0;
						local_kd_tree.enable_node(tail_node);
					}
					else
					{
						nearest_neighbor = local_kd_tree.nearests(node, 1, &self.tsp_data).first().unwrap().0;
					}

					// Remove nodes from the KD tree if they already have a
					// degree greater than 0
					if degree[node.id] > 0
					{
						local_kd_tree.disable_node(node);
					}

					if degree[nearest_neighbor.id] > 0
					{
						local_kd_tree.disable_node(&nearest_neighbor);
					}

					// Increase degrees
					degree[node.id            ] = degree[node.id            ] + 1;
					degree[nearest_neighbor.id] = degree[nearest_neighbor.id] + 1;

					// Insert edge
					let new_edge = BBEdge::new(&self.tsp_data, node.id, nearest_neighbor.id);
					new_tour.add(new_edge);

					// Handle the tails
					if !(tails[node.id] < TSPNodeID::MAX)
					{
						if !(tails[nearest_neighbor.id] < TSPNodeID::MAX)
						{
							// Both don't have a tail yet
							tails[node.id]             = nearest_neighbor.id;
							tails[nearest_neighbor.id] = node.id;
						}
						else
						{
							// node does not have a tail yet but its partner does
							let tail = tails[nearest_neighbor.id];
							tails[node.id] = tails[nearest_neighbor.id];
							tails[tail]    = node.id;
						}
					}
					else if !(tails[nearest_neighbor.id] < TSPNodeID::MAX)
					{
						// node does have a tail but its partner hasn't yet
						let tail = tails[node.id];
						tails[tail]                = nearest_neighbor.id;
						tails[nearest_neighbor.id] = tails[node.id];
					}
					else
					{
						// Both already have tails
						let tail_1 = tails[node.id];
						let tail_2 = tails[nearest_neighbor.id]; 
						tails[tail_1] = tails[nearest_neighbor.id];
						tails[tail_2] = tails[node.id];
					}
				}
			}
		}

		// Sanity check
		let bad_nodes = self.tsp_data.nodes.iter()
			.filter(|node| degree[node.id] > 2)
			.collect::<Vec<&TSPNode>>();

		assert!(bad_nodes.len() == 0);

		// Find the two nodes that haven't been connected yet via an edge
		let lonely_nodes = self.tsp_data.nodes.iter()
			.filter(|node| degree[node.id] < 2)
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