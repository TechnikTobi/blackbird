use std::cmp::Ordering;

use crate::tsp_lib::node::TSPNode;

use super::kd_tree::E_SPLIT_AXIS;

#[derive(Clone, Copy)]
pub struct
KDtreeNodeData
{
	pub tsp_node:                      TSPNode,
	pub enabled:                       bool
}

impl
KDtreeNodeData
{
	pub fn
	new
	(
		tsp_node:                      TSPNode
	)
	-> KDtreeNodeData
	{
		KDtreeNodeData 
		{ 
			tsp_node:                  tsp_node,
			enabled:                   true
		}
	}

	pub fn
	axis_compare
	(
		&self,
		other:                         &KDtreeNodeData,
		axis:                          E_SPLIT_AXIS
	)
	-> Ordering
	{
		match axis
		{
			E_SPLIT_AXIS::X => self.tsp_node.x.partial_cmp(&other.tsp_node.x).unwrap(),
			E_SPLIT_AXIS::Y => self.tsp_node.y.partial_cmp(&other.tsp_node.y).unwrap(),
		}
	}
}