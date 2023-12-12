use std::hash::Hasher;
use std::hash::Hash;

use crate::tsp_lib::node::*;
use crate::tsp_lib::data::*;

/// The BBEdge 
#[derive(Clone, Copy, Debug)]
pub struct
BBEdge
{
	pub start:                         TSPNodeID,
	pub end:                           TSPNodeID,
	pub weight:                        TSPWeight
}

#[derive(PartialEq, Eq)]
pub enum
EEdgeMarking
{
	ADDED,
	DELETED,
	NONE
}

impl
BBEdge
{
	/// Create a new edge based on two node IDs and the data struct containing
	/// all the information required for computing the distance between the two
	pub fn
	new
	(
		tsp_data:                      &TSPData,
		node1:                         TSPNodeID,
		node2:                         TSPNodeID,
	)
	-> BBEdge
	{
		// Make sure that we don't have a loop
		assert!(node1 != node2);
		
		BBEdge
		{
			start:  node1,
			end:    node2,
			weight: tsp_data.get_distance_between_via_id(node1, node2)
		}
	}

	/// Gets specifically used for edge deletion management during the heuristic
	pub fn
	new_weightless
	(
		node1:                         TSPNodeID,
		node2:                         TSPNodeID,
	)
	-> BBEdge
	{
		// Make sure that we don't have a loop
		assert!(node1 != node2);

		BBEdge
		{
			start:  node1,
			end:    node2,
			weight: -1.0
		}
	}

	/// Simply gets the length/weight/cost of the edge
	pub fn
	len
	(
		&self
	)
	-> TSPWeight
	{
		self.weight
	}

	#[allow(dead_code)]
	pub fn
	print
	(
		&self
	)
	{
		println!("{} -- {}", self.start, self.end);
	}

	#[allow(dead_code)]
	pub fn
	print_debug
	(
		&self
	)
	{
		println!("{} -- {} weight: {}", self.start, self.end, self.weight);
	}
}

impl
PartialEq 
for 
BBEdge
{
	fn
	eq
	(
		&self,
		other: &Self
	)
	-> bool
	{
		(self.start == other.start && self.end == other.end)
		|| (self.start == other.end && self.end == other.start)
	}
}

impl Eq for BBEdge {}

impl 
Hash 
for 
BBEdge 
{
	fn 
	hash<H: Hasher>
	(
		&self, 
		state: &mut H
	)
	{
		if self.start < self.end
		{
			(self.start, self.end).hash(state);
		}
		else
		{
			(self.end, self.start).hash(state);
		}
	}
}