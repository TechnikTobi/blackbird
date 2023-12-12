use crate::tsp_lib::node::{TSPWeight, TSPNodeID};

use super::bb_edge::BBEdge;

/// This data structure gets used as intermediate representation after the tour
/// initialization. Afterwards the tour gets stored using a BBFlipper or a vector
pub struct
BBTour
{
	pub edges:                         Vec<BBEdge>,
}

impl
BBTour
{
	/// Creates a new, empty BBTour struct
	pub fn
	new
	()
	-> BBTour
	{
		BBTour {
			edges:                     Vec::new()
		}
	}

	/// Computes the length of the tour - does NOT check if this is actually a
	/// valid cycle or not
	pub fn
	compute_len
	(
		&self
	)
	-> TSPWeight
	{
		self.edges.iter().map(|edge| edge.len()).sum()
	}

	/// Add an edge to the BBTour struct
	pub fn
	add
	(
		&mut self,
		edge: BBEdge,
	)
	{
		self.edges.push(edge)
	}

	/// Checks if the set of edges stored in the struct actually form a valid
	/// tour
	pub fn
	is_valid
	(
		&self,
		do_print: bool,
	)
	-> Option<Vec<TSPNodeID>>
	{
		// An empty tour can't be valid
		if self.edges.is_empty()
		{
			return None;
		}

		// Remember which nodes have been visited
		let mut visited_nodes = Vec::new();

		// Start with the first edge
		let mut current_node_id = self.edges.first().unwrap().start;

		loop
		{
			// Add visited node
			visited_nodes.push(current_node_id);
			if do_print { print!(" -- {}", current_node_id); }

			// Get all edges that are adjacent to the current node
			let next_edge_candidates = self.edges.iter().filter(
				|edge| edge.start == current_node_id || edge.end == current_node_id
			).collect::<Vec::<&BBEdge>>();

			// There need to be exactly two edges that are adjacent to this node
			if next_edge_candidates.len() != 2
			{
				return None;
			}

			// Unpack the two edges
			let edge_one = next_edge_candidates[0];
			let edge_two = next_edge_candidates[1];

			if edge_one.start == current_node_id
			{
				// edge_one.end is the "other" node
				if visited_nodes.contains(&edge_one.end)
				{
					// edge_one has been visited - edge_two needs to be unvisited
				}
				else
				{
					current_node_id = edge_one.end;
					continue;
				}
			}
			else if edge_one.end == current_node_id
			{
				// edge_one.start is the "other" node
				if visited_nodes.contains(&edge_one.start)
				{
					// edge_one has been visited - edge_two needs to be unvisited
				}
				else
				{
					current_node_id = edge_one.start;
					continue;
				}
			}
			else
			{
				return None;
			}

			// edge_one has been visited - check edge_two

			if edge_two.start == current_node_id
			{
				// edge_two.end is the "other" node
				if visited_nodes.contains(&edge_two.end)
				{
					// Both edges have been visited
					// If at this point all nodes have been visited, i.e. their
					// number equals the number of edges, we have a valid tour
					if visited_nodes.len() == self.edges.len()
					{
						return Some(visited_nodes);
					}
					else
					{
						return None;
					}
				}
				else
				{
					current_node_id = edge_two.end;
					continue;
				}
			}
			else if edge_two.end == current_node_id
			{
				// edge_two.start is the "other" node
				if visited_nodes.contains(&edge_two.start)
				{
					// Both edges have been visited
					// If at this point all nodes have been visited, i.e. their
					// number equals the number of edges, we have a valid tour
					if visited_nodes.len() == self.edges.len()
					{
						return Some(visited_nodes);
					}
					else
					{
						return None;
					}
				}
				else
				{
					current_node_id = edge_two.start;
					continue;
				}
			}
			else
			{
				return None;
			}
		}
	}
}