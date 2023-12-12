// Type Definitions
pub type TSPNodeID = usize;
pub type TSPWeight = f64;

/// A simple data structure for representing one of the many nodes of a TSP 
/// problem instance with its identifier and (2D) coordinates
#[derive(Copy, Clone, Debug)]
pub struct
TSPNode
{
	pub id: TSPNodeID,
	pub x:  TSPWeight,
	pub y:  TSPWeight,
}

impl
TSPNode
{
	pub fn
	new
	(
		id: TSPNodeID,
		x:  TSPWeight,
		y:  TSPWeight
	)
	-> TSPNode
	{
		TSPNode {id: id, x: x, y: y}
	}

	pub fn
	print
	(
		&self
	)
	{
		println!("{:>10}: {:>10} {:>10}", self.id, self.x, self.y);
	}
}