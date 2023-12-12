use std::collections::HashMap;
use crate::blackbird::data::bb_data::BBData;
use crate::tsp_lib::node::TSPNodeID;

impl 
BBData
{
	/// This computes the sparse edge set used by the heuristic at multiple
	/// points.
	pub fn
	sparse_edge_map
	(
		&mut self

	)
	{
		if self.kd_tree.is_none()
		{
			// Construct the KD tree if it does not exist yet
			self.construct_kd_tree();
		}

		// Collect in the following table the good edges
		// The key marks on of the nodes of an edge, the value is a list of 
		// end nodes of the good edges, combined with the weight of the edge
		let mut sparse_edge_map: HashMap<TSPNodeID, Vec<TSPNodeID>> = HashMap::new();

		let k = self.cli_args.quadrant_nearest_count;
		let goal = 4 * k; // x4 due to quadrants

		// Compute the good edges for all nodes
		for node_id in 0..self.tsp_data.n
		{
			let mut node_specific_sparse_edge_map = sparse_edge_map.get(&node_id).unwrap_or(&Vec::new()).clone();	

			// Get for each quadrant the k nearest
			let other_nodes: Vec<TSPNodeID> = self.kd_tree.as_ref().unwrap().all_quadrant_nearest(
				&self.tsp_data.nodes[node_id], 
				k,
				&self.tsp_data
			).iter().map(|&(node, _)| node.id).collect();

			// Add to the vector of this node and the other node
			for other_node_id in &other_nodes[..std::cmp::min(goal, other_nodes.len())]
			{
				if other_node_id < &node_id
				{
					// For the other node
					if let Some(other_node_vec) = sparse_edge_map.get(&other_node_id)
					{
						if !other_node_vec.contains(&node_id)
						{
							let mut temp = other_node_vec.clone();
							temp.push(node_id);
							sparse_edge_map.insert(*other_node_id, temp);
						}
					}
					else
					{
						sparse_edge_map.insert(*other_node_id, vec![node_id]);
					}
				}
				else
				{
					// For this node
					if !node_specific_sparse_edge_map.contains(other_node_id)
					{
						node_specific_sparse_edge_map.push(*other_node_id);
					}
				}
				
			}

			sparse_edge_map.insert(node_id, node_specific_sparse_edge_map);
		}

		// Sort the good edges by their weight
		for node_id in 0..self.tsp_data.n
		{
			let mut to_be_sorted = sparse_edge_map[&node_id].clone();
			to_be_sorted.sort_by(|a, b| 
				self.tsp_data.get_distance_between_via_id(node_id, *a)
				.partial_cmp(
					&self.tsp_data.get_distance_between_via_id(node_id, *b)
				).unwrap()
			);

			sparse_edge_map.insert(node_id, to_be_sorted);
		}

		self.sparse_edge_map = sparse_edge_map;

		self.make_symmetric();
	}

	/// Due to how the sparse edge map is generated and subsequently accessed
	/// it is required that the mappings are symmetric, e.g. if y is element
	/// of the vector at x, then x should also be element of the vector at y
	/// This symmetry gets established via this function
	pub fn
	make_symmetric
	(
		&mut self
	)
	{
		let keys = self.sparse_edge_map.keys().map(|a| *a).collect::<Vec<TSPNodeID>>();
		for node_id in keys
		{
			let other_nodes = self.sparse_edge_map[&node_id].clone();
			for other_node_id in other_nodes
			{
				let mut other_node_list = self.sparse_edge_map[&other_node_id].clone();
				if !other_node_list.contains(&node_id)
				{
					other_node_list.push(node_id);
					other_node_list.sort_by(|a, b| 
						self.tsp_data.get_distance_between_via_id(other_node_id, *a)
						.partial_cmp(
							&self.tsp_data.get_distance_between_via_id(other_node_id, *b)
						).unwrap()
					);
					self.sparse_edge_map.insert(other_node_id, other_node_list);
				}
			}
		}
	}
}

