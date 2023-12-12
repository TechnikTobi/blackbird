use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Weak;
use std::rc::Rc;

use crate::tsp_lib::data::TSPData;
use crate::tsp_lib::node::TSPNodeID;
use crate::tsp_lib::node::TSPWeight;

/// Struct for storing a flip on the internal flip stack of the BBFlipper
pub struct
BBFlip
{
	x:                                 TSPNodeID,
	y:                                 TSPNodeID,
}

/// The main flipper structure that supports flip, next, prev, sequence, ...
pub struct
BBFlipper
{
	children:                          Vec<Rc<RefCell<BBFlipperNode>>>,
	flips:                             Vec<BBFlip>,
	pub total_flips:                   usize,
	pub total_unflips:                 usize,
}

/// A node of the flipper that stores the information of a single TSPNode
/// This includes explicit pointers to the next and previous node in the
/// flipper's tour
#[derive(Debug)]
pub struct
BBFlipperNode
{
	tsp_node_id:                       TSPNodeID,
	reversed:                          bool,
	left:                              Weak<RefCell<BBFlipperNode>>,
	right:                             Weak<RefCell<BBFlipperNode>>,
}

impl
BBFlipper
{
	/// Create a new flipper for a given tour that is represented in cycle form
	/// using a vector of TSPNodeIDs
	pub fn
	new
	(
		tour:                          &Vec<TSPNodeID>
	)
	-> Self
	{
		// The number of nodes will be needed multiple times
		let n = tour.len();

		// Create the child nodes
		let mut children = Vec::new();
		for tsp_node_id in 
			(
				*tour.iter().min().unwrap()..=*tour.iter().max().unwrap()
			)
			.collect::<Vec::<usize>>()
			.iter()
		{
			children.push(
				Rc::new(
					RefCell::new(
						BBFlipperNode
						{ 
							tsp_node_id:    *tsp_node_id,
							reversed:       false,
							left:           Weak::new(),
							right:          Weak::new(),
						}
					)
				)
			);
			assert_eq!(children.len()-1, *tsp_node_id);
		}

		// Setting pointers to the right
		for (tour_entry_index, node_id) in tour.iter().enumerate()
		{
			let next_node_id = tour[(tour_entry_index+1) % n];
			children[*node_id].borrow_mut().right = Rc::downgrade(&children[next_node_id]);
		}

		// Setting pointers to the left
		for (tour_entry_index, node_id) in tour.iter().enumerate()
		{
			let prev_node_id = tour[(tour_entry_index + n - 1) % n];
			children[*node_id].borrow_mut().left = Rc::downgrade(&children[prev_node_id]);
		}

		BBFlipper 
		{ 
			children:                  children,
			flips:                     Vec::new(),
			total_flips: 0,
			total_unflips: 0,
		}
	}

	/// Get the Flipper Node for a given TSPNodeID. Used only internally. 
	fn
	get
	(
		&self,
		tsp_node_id:                   &TSPNodeID,
	)
	-> Weak<RefCell<BBFlipperNode>>
	{
		Rc::downgrade(&self.children[*tsp_node_id])
	}

	/// Get the TSPNodeID of the node subsequent node in the current tour
	pub fn
	next
	(
		&self,
		tsp_node_id:                   &TSPNodeID
	)
	-> TSPNodeID
	{
		self.get(tsp_node_id).upgrade().unwrap().as_ref().borrow().next_id()
	}

	/// Get the TSPNodeID of the node preceeding node in the current tour
	pub fn
	prev
	(
		&self,
		tsp_node_id:                   &TSPNodeID
	)
	-> TSPNodeID
	{
		self.get(tsp_node_id).upgrade().unwrap().as_ref().borrow().prev_id()
	}

	/// Check if three nodes form a sequence in the current tour
	pub fn
	sequence
	(
		&self,
		start:                         &TSPNodeID,
		middle:                        &TSPNodeID,
		end:                           &TSPNodeID,
	)
	-> bool
	{
		let mut current = self.get(start);
		loop 
		{
			current = current.upgrade().unwrap().as_ref().borrow().next();

			let current_id = current.upgrade().unwrap().as_ref().borrow().tsp_node_id;

			if &current_id == start  { panic!("This should not happen when calling sequence"); }
			if &current_id == middle { return true; }
			if &current_id == end    { return false; }
		}
	}

	/// Perform a flip on the tour and keep track of it on the internal flip stack
	pub fn
	flip
	(
		&mut self,
		x:                             TSPNodeID,
		y:                             TSPNodeID,	
	)
	{
		self.flips.push(BBFlip { x: x, y: y });
		self.internal_flip(&x, &y);
		self.total_flips += 1;
	}

	/// Undo a flip. Checks that this undos the most recent flip performed by
	/// comparing with the top of the flip stack
	pub fn
	unflip
	(
		&mut self,
		x:                             TSPNodeID,
		y:                             TSPNodeID,	
	)
	{
		assert!(self.flips.len() > 0);
		assert_eq!(self.flips.last().unwrap().x, x);
		assert_eq!(self.flips.last().unwrap().y, y);

		self.flips.pop();

		self.internal_flip(&y, &x);
		self.total_unflips += 1;
	}

	/// The internal function for performing a flip. This can only be called
	/// via the publicly available flip and unflip methods
	fn
	internal_flip
	(
		&mut self,
		x:                             &TSPNodeID,
		y:                             &TSPNodeID,
	)
	{
		assert!(x != y);

		// Special cases
		if (&self.next(x) == y) || (&self.next(y) == x)
		{
			let start = if (&self.next(x) == y) { x } else { y };
			let end   = if (&self.next(x) == y) { y } else { x };

			let start_prev = self.get(start).upgrade().unwrap().as_ref().borrow().prev();
			let end_next   = self.get(end  ).upgrade().unwrap().as_ref().borrow().next();

			self.get(start).upgrade().unwrap().as_ref().borrow_mut().set_next(end_next.clone());
			self.get(start).upgrade().unwrap().as_ref().borrow_mut().set_prev(self.get(end));
			
			self.get(end).upgrade().unwrap().as_ref().borrow_mut().set_prev(start_prev.clone());
			self.get(end).upgrade().unwrap().as_ref().borrow_mut().set_next(self.get(start));

			start_prev.upgrade().unwrap().as_ref().borrow_mut().set_next(self.get(end));
			end_next.  upgrade().unwrap().as_ref().borrow_mut().set_prev(self.get(start));

			return;
		}

		let mut current_node = self.get(&self.next(x));

		while (&current_node.upgrade().unwrap().as_ref().borrow().tsp_node_id != y)
		{
			current_node.upgrade().unwrap().as_ref().borrow_mut().flip();

			// Need to get the previous node because previously, this was the
			// next node. This is due to the flipping. 
			current_node = current_node.upgrade().unwrap().as_ref().borrow().prev();
		}

		let x_prev = self.get(x).upgrade().unwrap().as_ref().borrow().prev();
		let y_next = self.get(y).upgrade().unwrap().as_ref().borrow().next();

		// Update X and Y
		self.get(x).upgrade().unwrap().as_ref().borrow_mut().flip();
		self.get(y).upgrade().unwrap().as_ref().borrow_mut().flip();

		self.get(x).upgrade().unwrap().as_ref().borrow_mut().set_next(y_next.clone());
		self.get(y).upgrade().unwrap().as_ref().borrow_mut().set_prev(x_prev.clone());

		// Update x_prev & y_next
		x_prev.upgrade().unwrap().as_ref().borrow_mut().set_next(self.get(y));
		y_next.upgrade().unwrap().as_ref().borrow_mut().set_prev(self.get(x));
	}

	#[allow(dead_code)]
	pub fn
	print_debug
	(
		&self
	)
	{
		let mut visited = HashSet::new();
		
		let start_id = self.get(&0).upgrade().unwrap().as_ref().borrow().tsp_node_id;
		visited.insert(start_id);

		let mut current_id = self.next(&start_id);
		visited.insert(current_id);

		println!("\nFLIPPER DEBUG OUTPUT START");
		while start_id != current_id
		{
			println!("{} ", current_id);
			current_id = self.next(&current_id);

			if visited.contains(&current_id) && current_id != start_id
			{
				panic!("{:?} already contains {}", visited, current_id);
			}

			visited.insert(current_id);
		}
		println!("\nFLIPPER DEBUG OUTPUT END");
	}	

	/// Converts the tour currently stored by the flipper into a cycle that
	/// is represented via a vector
	pub fn
	as_cycle
	(
		&self
	)
	-> Vec<TSPNodeID>
	{
		let start_node = 0;
		let mut current_node = self.next(&start_node);

		let mut cycle = Vec::with_capacity(self.children.len());
		cycle.push(start_node);

		while current_node != start_node
		{
			assert!(!cycle.contains(&current_node));
			cycle.push(current_node);
			current_node = self.next(&current_node);
		}

		assert_eq!(cycle.len(), self.children.len());

		return cycle;
	}

	/// Computes the cost of the tour currently stored by the flipper
	pub fn
	cost
	(
		&self,
		tsp_data:                      &TSPData,
	)
	-> TSPWeight
	{
		let cycle = self.as_cycle();
		let mut length = 0.0;
		for i in 0..cycle.len()
		{
			length += tsp_data.get_distance_between_via_id(cycle[i], cycle[(i+1)%tsp_data.n]);
		}

		return length;
	}
}

impl
BBFlipperNode
{
	/// Gets the next flipper node
	/// Depends on the internal reversed flag 
	fn
	next
	(
		&self
	)
	-> Weak<RefCell<BBFlipperNode>>
	{
		if self.reversed { self.left.clone() } else { self.right.clone() }
	}

	/// Gets the previous flipper node
	/// Depends on the internal reversed flag 
	fn
	prev
	(
		&self
	)
	-> Weak<RefCell<BBFlipperNode>>
	{
		if self.reversed { self.right.clone() } else { self.left.clone() }
	}

	/// Gets the TSPNodeID of the next flipper node
	/// Depends on the internal reversed flag 
	fn
	next_id
	(
		&self
	)
	-> TSPNodeID
	{
		self.next().upgrade().unwrap().as_ref().borrow().tsp_node_id
	}

	/// Gets the TSPNodeID of the previous flipper node
	/// Depends on the internal reversed flag 
	fn
	prev_id
	(
		&self
	)
	-> TSPNodeID
	{
		self.prev().upgrade().unwrap().as_ref().borrow().tsp_node_id
	}

	/// Set the pointer to the next flipper node
	fn
	set_next
	(
		&mut self,
		new_next:                      Weak<RefCell<BBFlipperNode>>,
	)
	{
		if self.reversed { self.left = new_next } else { self.right = new_next };
	}


	fn
	set_prev
	(
		&mut self,
		new_next:                      Weak<RefCell<BBFlipperNode>>,
	)
	{
		if self.reversed { self.right = new_next } else { self.left = new_next };
	}

	/// Flips the internal reversed flag 
	fn
	flip
	(
		&mut self
	)
	{
		self.reversed = !self.reversed;
	}
}