use std::cmp::Ordering;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::blackbird::data::bb_data::BBData;
use crate::util::tsp_weight_min;
use crate::util::tsp_weight_max;
use crate::tsp_lib::data::TSPData;
use crate::tsp_lib::node::*;

use super::kd_node::KDtreeNodeData;

#[derive(Clone)]
pub struct
KDtree
{
	root:                              Option<KDtreeNodeData>,
	cut_axis:                          Option<E_SPLIT_AXIS>,
	bucket:                            Option<Vec<KDtreeNodeData>>,
	l_child:                           Option<Box<KDtree>>,
	r_child:                           Option<Box<KDtree>>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum
E_SPLIT_AXIS
{
	X,
	Y,
}

impl
KDtree
{
	const CUTOFF: usize = 5;           // When to use buckets

	/// Creates a new KD tree for a given TSPData struct, using the nodes
	/// and distance function provided by said struct. This also expects a 
	/// random generator for making a call to to this function reproducible.
	/// This is construction is done recursively, splitting the data at each 
	/// level. At some point, when there is not enough data (as defined by the
	/// CUTOFF constant), a bucket is used instead of individual nodes. 
	pub fn
	new<'a>
	(
		tsp_data:                      &'a TSPData,
		random_generator:              &mut StdRng,
	)
	-> KDtree
	{
		// Construct the KD node data
		let kd_nodes: Vec<_> = tsp_data.nodes
			.iter()
			.map(|tsp_node| KDtreeNodeData::new(tsp_node.clone()))
			.collect();
		
		// Call the recursive construction method
		let kd_tree = Self::recursive_new(
			tsp_data,
			&kd_nodes,
			0,
			random_generator
		);

		return kd_tree;
	}

	fn
	recursive_new
	(
		tsp_data:                      &TSPData,
		kd_nodes:                      &Vec<KDtreeNodeData>,
		depth:                         usize,
		random_generator:              &mut StdRng,
	)
	-> KDtree
	{
		
		if kd_nodes.len() < Self::CUTOFF
		{
			// Return a leaf node that has data in its bucket
			return KDtree 
			{ 
				root:                  None, 
				cut_axis:              None,
				l_child:               None,
				r_child:               None,
				bucket:                Some(kd_nodes.clone()),
			}
		}
		else // Too many data points to handle, need to further subdivide
		{

			// Determine cut axis and node
			let axis = Self::determine_split_axis(kd_nodes);

			let cut_node = Self::determine_cut_node(
				kd_nodes, 
				axis, 
				random_generator
			);

			// Split the nodes along the axis
			let left_subset: Vec<_> = kd_nodes
				.iter()
				.filter(
					|kd_node| 
					kd_node.axis_compare(&cut_node, axis) == Ordering::Less &&
					kd_node.tsp_node.id != cut_node.tsp_node.id
				)
				.map(|node| node.clone())
				.collect();
			let right_subset: Vec<_> = kd_nodes
				.iter()
				.filter(
					|kd_node| 
					kd_node.axis_compare(&cut_node, axis) != Ordering::Less &&
					kd_node.tsp_node.id != cut_node.tsp_node.id
				)
				.map(|node| node.clone())
				.collect();

			// Build the two subtrees that form the children
			let left_child = Self::recursive_new(
				tsp_data, 
				&left_subset, 
				depth+1,
				random_generator
			);

			let right_child = Self::recursive_new(
				tsp_data, 
				&right_subset, 
				depth+1,
				random_generator
			);

			// Construct the tree with the cut node as root
			return KDtree
			{ 
				root:                  Some(cut_node), 
				cut_axis:              Some(axis),
				l_child:               Some(Box::new(left_child)), 
				r_child:               Some(Box::new(right_child)), 
				bucket:                None,
			}
	
		}
		
	}

	/// Determines the axis along which the split needs to be performed for a
	/// given set of data
	fn
	determine_split_axis
	(
		data:                          &Vec<KDtreeNodeData>,
	)
	-> E_SPLIT_AXIS
	{
		let mut min_x = TSPWeight::MAX;
		let mut min_y = TSPWeight::MAX;

		let mut max_x = TSPWeight::MIN;
		let mut max_y = TSPWeight::MIN;

		for node in data
		{
			min_x = tsp_weight_min(node.tsp_node.x, min_x);
			min_y = tsp_weight_min(node.tsp_node.y, min_y);

			max_x = tsp_weight_max(node.tsp_node.x, max_x);
			max_y = tsp_weight_max(node.tsp_node.y, max_y);
		}

		if (max_x - min_x).abs() > (max_y - min_y).abs()
		{
			return E_SPLIT_AXIS::X;
		}
		else
		{
			return E_SPLIT_AXIS::Y;
		}
	}

	/// Once the axis has been determined, this function computes the cut node
	/// itself, which is/should be the median of the given data along the
	/// specified axis.
	/// Note that this does not necessarily need to be the perfect median but
	/// can also be approximated by computing the median of some smaller 
	/// randomly selected subset
	fn
	determine_cut_node
	(
		data:                          &Vec<KDtreeNodeData>,
		axis:                          E_SPLIT_AXIS,
		random_generator:              &mut StdRng,
	)
	-> KDtreeNodeData
	{
		// How many random samples to draw for median computation
		let number_of_random_samples = 100;

		// Draw random samples
		let mut samples: Vec<KDtreeNodeData> = data
			.choose_multiple(random_generator, number_of_random_samples)
			.map(|node| node.clone())
			.collect();

		// Sort the randomly drawn samples so that the median can be selected
		samples.sort_by(
			|a, b|
			match axis
			{
				E_SPLIT_AXIS::X => a.tsp_node.x.partial_cmp(&b.tsp_node.x).unwrap(),
				E_SPLIT_AXIS::Y => a.tsp_node.y.partial_cmp(&b.tsp_node.y).unwrap(),
			}
		);

		// Approximate the median node
		return samples[samples.len()/2];
	}

	/// Gets count-many nodes that are nearest to the given tsp_node, based
	/// on the distance defined in tsp_data. These are combined with their
	/// distance to tsp_node and sorted according to these in ascending order
	pub fn
	nearests
	(
		&self,
		tsp_node:                      &TSPNode,
		count:                         usize,
		tsp_data:                      &TSPData // Needed for distance computations
	)
	-> Vec<(TSPNode, TSPWeight)>
	{
		return self.internal_nearests(tsp_node, count, &LocalBounds::new(), TSPWeight::MAX, tsp_data);
	}

	/// Recursive function for determining the nearest nodes
	fn
	internal_nearests
	(
		&self,
		tsp_node:                      &TSPNode,
		count:                         usize,
		bounds:                        &LocalBounds,
		current_radius:                TSPWeight,
		tsp_data:                      &TSPData // Needed for distance computations
	)
	-> Vec<(TSPNode, TSPWeight)>
	{
		let mut nearests_nodes = Vec::new();

		if let Some(unpacked_root) = self.root
		{
			// First of all, add the unpacked root if it is different from 
			// the given tsp_node AND is enabled AND within the bounds
			let distance_to_root = tsp_data.get_distance_between(&unpacked_root.tsp_node, tsp_node);
			
			if 
				   unpacked_root.tsp_node.id != tsp_node.id
				&& unpacked_root.enabled
				&& bounds.contains(&unpacked_root.tsp_node)
			{
				nearests_nodes.push((
					unpacked_root.tsp_node.clone(),
					distance_to_root
				));
			}

			// Now, let us see which child of the root to visit first
			// This is done by checking on which side of the root node the 
			// given tsp_node would be located
			let comparison_with_root = KDtreeNodeData::new(*tsp_node).axis_compare(&unpacked_root, self.cut_axis.unwrap());

			if comparison_with_root == Ordering::Less
			{
				nearests_nodes.append(&mut self.l_child.as_ref().unwrap().internal_nearests(tsp_node, count, bounds, current_radius, tsp_data));
			}
			else
			{
				nearests_nodes.append(&mut self.r_child.as_ref().unwrap().internal_nearests(tsp_node, count, bounds, current_radius, tsp_data));
			}

			// Get the largest distance currently known in nearest nodes
			// Also considering the current best known radius to use
			nearests_nodes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
			let radius = if nearests_nodes.len() > 0
			{
				tsp_weight_min(
					current_radius, 
					if nearests_nodes.len() >= count { nearests_nodes[count-1].1 } else { nearests_nodes.last().unwrap().1 }
				)
			}
			else
			{
				current_radius
			};

			// Check with the bounds at this node
			let needs_visit_on_other_side;
			if comparison_with_root == Ordering::Less
			{
				match self.cut_axis.unwrap()
				{
					E_SPLIT_AXIS::X => {
						needs_visit_on_other_side = tsp_node.x + radius >= unpacked_root.tsp_node.x || (bounds.are_bounded() && nearests_nodes.len() < count);
					},
					E_SPLIT_AXIS::Y => {
						needs_visit_on_other_side = tsp_node.y + radius >= unpacked_root.tsp_node.y || (bounds.are_bounded() && nearests_nodes.len() < count);
					},
				}
			}
			else
			{
				match self.cut_axis.unwrap()
				{
					E_SPLIT_AXIS::X => {
						needs_visit_on_other_side = tsp_node.x - radius < unpacked_root.tsp_node.x || (bounds.are_bounded() && nearests_nodes.len() < count);
					},
					E_SPLIT_AXIS::Y => {
						needs_visit_on_other_side = tsp_node.y - radius < unpacked_root.tsp_node.y || (bounds.are_bounded() && nearests_nodes.len() < count);
					},
				}
			}

			// If the bounds with the current radius are surpassed, we need to 
			// visit the other child node as well.
			if needs_visit_on_other_side
			{
				if comparison_with_root == Ordering::Less
				{
					nearests_nodes.append(&mut self.r_child.as_ref().unwrap().internal_nearests(tsp_node, count, bounds, radius, tsp_data));
				}
				else
				{
					nearests_nodes.append(&mut self.l_child.as_ref().unwrap().internal_nearests(tsp_node, count, bounds, radius, tsp_data));
				}
			}
		}
		else // We have a bucket
		{
			for kd_node in self.bucket.as_ref().unwrap()
			{
				if 
				(
					!kd_node.enabled 
					|| kd_node.tsp_node.id == tsp_node.id
					|| !bounds.contains(&kd_node.tsp_node)
				)
				{
					// Ignore this node as it is currently disabled for queries
					// Or the given node itself (for which we know that the
					// distance is zero)
					continue;
				}

				let distance = tsp_data.get_distance_between(tsp_node, &kd_node.tsp_node);
				
				nearests_nodes.push((kd_node.tsp_node.clone(), distance));
			}
		}

		nearests_nodes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

		return nearests_nodes
			.iter()
			.take(count)
			.map(|element| element.clone())
			.collect();
	}

	pub fn
	disable_node
	(
		&mut self,
		node:                          &TSPNode
	)
	{
		self.change_node_enable(node, false);
	}

	pub fn
	enable_node
	(
		&mut self,
		node:                          &TSPNode
	)
	{
		self.change_node_enable(node, true);
	}

	fn
	change_node_enable
	(
		&mut self,
		node:                          &TSPNode,
		new_enable_value:              bool,
	)
	{
		if self.root.is_some()
		{
			if self.root.unwrap().tsp_node.id == node.id
			{
				self.root.as_mut().unwrap().enabled = new_enable_value;
			}

			// node is not the root, so let's see if we have to go left or right
			let comparison_with_root = KDtreeNodeData::new(*node).axis_compare(&self.root.unwrap(), self.cut_axis.unwrap());

			if comparison_with_root == Ordering::Less
			{
				self.l_child.as_deref_mut().unwrap().change_node_enable(node, new_enable_value);
			}
			else
			{
				self.r_child.as_deref_mut().unwrap().change_node_enable(node, new_enable_value);
			}
		}
		else
		{
			for mut kd_node in self.bucket.as_mut().unwrap().iter_mut()
			{
				if kd_node.tsp_node.id == node.id
				{
					kd_node.enabled = new_enable_value;
					return;
				}
			}
		}
	}


	pub fn
	all_quadrant_nearest
	(
		&self,
		tsp_node:                      &TSPNode,	
		count:                         usize,
		tsp_data:                      &TSPData // Needed for distance computations
	)
	-> Vec<(TSPNode, TSPWeight)>
	{
		// Variable setup
		let mut local_bounds;
		let mut nearests = Vec::new();

		// Check the right upper corner
		local_bounds = LocalBounds::new();
		local_bounds.x_lower = tsp_node.x;
		local_bounds.y_lower = tsp_node.y;
		nearests.append(&mut self.internal_nearests(tsp_node, count, &local_bounds, TSPWeight::MAX, tsp_data));

		// Check the right lower corner
		local_bounds = LocalBounds::new();
		local_bounds.x_lower = tsp_node.x;
		local_bounds.y_upper = tsp_node.y;
		nearests.append(&mut self.internal_nearests(tsp_node, count, &local_bounds, TSPWeight::MAX, tsp_data));

		// Check the left lower corner
		local_bounds = LocalBounds::new();
		local_bounds.x_upper = tsp_node.x;
		local_bounds.y_upper = tsp_node.y;
		nearests.append(&mut self.internal_nearests(tsp_node, count, &local_bounds, TSPWeight::MAX, tsp_data));

		// Check the left upper corner
		local_bounds = LocalBounds::new();
		local_bounds.x_upper = tsp_node.x;
		local_bounds.y_lower = tsp_node.y;
		nearests.append(&mut self.internal_nearests(tsp_node, count, &local_bounds, TSPWeight::MAX, tsp_data));

		// Sort the nearest nodes
		nearests.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

		// Remove duplicates
		nearests.dedup_by_key(|(node, _)| node.id);

		return nearests;
	}
}


struct
LocalBounds
{
	pub x_lower:                             TSPWeight,
	pub x_upper:                             TSPWeight,
	pub y_lower:                             TSPWeight,
	pub y_upper:                             TSPWeight
}

impl
LocalBounds
{
	pub fn
	new
	()
	-> LocalBounds
	{
		LocalBounds 
		{ 
			x_lower: TSPWeight::MIN, 
			x_upper: TSPWeight::MAX, 
			y_lower: TSPWeight::MIN, 
			y_upper: TSPWeight::MAX 
		}
	}

	pub fn
	contains
	(
		&self,
		tsp_node:                      &TSPNode,	
	)
	-> bool
	{
		   self.x_lower <= tsp_node.x
		&& self.x_upper >= tsp_node.x
		&& self.y_lower <= tsp_node.y
		&& self.y_upper >= tsp_node.y
	}

	pub fn
	are_bounded
	(
		&self
	)
	-> bool
	{
		   self.x_lower > TSPWeight::MIN
		|| self.x_upper < TSPWeight::MAX
		|| self.y_lower > TSPWeight::MIN
		|| self.y_upper < TSPWeight::MAX
	}

}

impl
BBData
{
	/// Constructs the KD tree for the given TSP data. If the KD tree already
	/// exists, the function skips the creation and immediately returns
	pub fn
	construct_kd_tree
	(
		&mut self
	)
	{
		// Check if KD tree has already been constructed - if so, return
		if self.kd_tree.is_some()
		{
			return;
		}

		// Start time measurement
		let time_measurement_start = Instant::now();

		// Construct the KD tree
		self.kd_tree = Some(KDtree::new(&self.tsp_data, &mut self.random_generator));

		println!("KD Tree Build Time : {}", (time_measurement_start.elapsed().as_micros() as f64) / 1000000.0);
	}
}