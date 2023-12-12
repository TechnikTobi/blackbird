use rand::Rng;

use crate::blackbird::data::bb_data::BBData;
use crate::tsp_lib::node::TSPNodeID;

/// This enum describes which kicking strategy to use when CLK is applied
/// Right now only "WALK" is implemented but CONCORDE provides some others as
/// well that were not implemented at this point.
pub enum
EKickType
{
	WALK,
}

impl
BBData
{
	/// The main function for performing a kick to the tour currently stored in 
	/// the flipper of the BBData struct
	pub fn
	kick
	(
		&mut self
	)
	{
		let kick_type = EKickType::WALK;
		let (
			t1, t2, 
			mut t3, mut t4, 
			mut t5, mut t6, 
			mut t7, mut t8
		) = match kick_type
		{
			EKickType::WALK   => {
				self.walk_kick()
			},
			_ => todo!(),
		};

		if !self.flipper.as_ref().unwrap().sequence(&t1, &t3, &t5)
		{
			(t3, t5) = (t5, t3);
			(t4, t6) = (t6, t4);
		}

		if !self.flipper.as_ref().unwrap().sequence(&t1, &t5, &t7)
		{
			(t5, t7) = (t7, t5);
			(t6, t8) = (t8, t6);

			if !self.flipper.as_ref().unwrap().sequence(&t1, &t3, &t5)
			{
				(t3, t5) = (t5, t3);
				(t4, t6) = (t6, t4);
			}
		}

		self.flipper.as_mut().unwrap().flip(t2, t5);
		self.flipper.as_mut().unwrap().flip(t3, t7);
		self.flipper.as_mut().unwrap().flip(t5, t6);

		self.add_node_and_neighbors_to_queue(t1, false);
		self.add_node_and_neighbors_to_queue(t2, true);
		self.add_node_and_neighbors_to_queue(t3, false);
		self.add_node_and_neighbors_to_queue(t4, true);
		self.add_node_and_neighbors_to_queue(t5, false);
		self.add_node_and_neighbors_to_queue(t6, true);
		self.add_node_and_neighbors_to_queue(t7, false);
		self.add_node_and_neighbors_to_queue(t8, true);

		// Updates the tour length caused by this kick
		// Not sure if this is really correct
		self.current_tour_length +=
			- self.tsp_data.get_distance_between_via_id(t1, t6)
			- self.tsp_data.get_distance_between_via_id(t2, t5)
			- self.tsp_data.get_distance_between_via_id(t3, t8)
			- self.tsp_data.get_distance_between_via_id(t4, t7)
			+ self.tsp_data.get_distance_between_via_id(t1, t2)
			+ self.tsp_data.get_distance_between_via_id(t3, t4)
			+ self.tsp_data.get_distance_between_via_id(t5, t6)
			+ self.tsp_data.get_distance_between_via_id(t7, t8);
	}

	/// Needed for the function that adds nodes (and their neighbors) that
	/// participated in a kick to the queue of nodes to be processed in the 
	/// next LK run
	const MARK_LEVEL: usize = 10;

	/// Adds the given node and its neighbors in one direcion (either towards
	/// next or prev) to the queue of nodes that require processing in the next
	/// LK run. The level of how deep this marking regarding neighbors goes is
	/// controlled by [MARK_LEVEL]
	fn
	add_node_and_neighbors_to_queue
	(
		&mut self,
		tx:                            TSPNodeID,
		to_next:                       bool,
	)
	{
		self.node_queue.push_back(tx);

		let mut k = tx;
		for _ in 0..Self::MARK_LEVEL
		{
			k = if (to_next) { self.flipper.as_ref().unwrap().next(&k) } else { self.flipper.as_ref().unwrap().prev(&k) };
			self.node_queue.push_back(k)
		}

		for other_node_id in &self.sparse_edge_map[&tx]
		{
			self.node_queue.push_back(*other_node_id);
		}
	}

	const WALK_STEPS: usize = 50;

	/// The walk kick, the default kick method employed by Concorde. The code 
	/// of this function is based heavily on the logic of the Concorde version. 
	fn
	walk_kick
	(
		&mut self
	)
	-> (TSPNodeID, TSPNodeID, TSPNodeID, TSPNodeID, TSPNodeID, TSPNodeID, TSPNodeID, TSPNodeID)
	{
		let (s1, s2) = self.first_kicker();
		let mut s3: TSPNodeID;
		let mut s4: TSPNodeID;
		let mut s5: TSPNodeID;
		let mut s6: TSPNodeID;
		let mut s7: TSPNodeID;
		let mut s8: TSPNodeID;

		loop
		{
			let mut old = TSPNodeID::MAX;
			let mut n   = s2;

			for _ in 0..Self::WALK_STEPS
			{
				let j = self.random_generator.gen_range(0..self.sparse_edge_map[&n].len());

				if old != self.sparse_edge_map[&n][j]
				{
					old = n;
					n = self.sparse_edge_map[&n][j]
				}
			}

			s3 = n;
			s4 = self.flipper.as_ref().unwrap().next(&s3);
			n  = s4;

			for _ in 0..Self::WALK_STEPS
			{
				let j = self.random_generator.gen_range(0..self.sparse_edge_map[&n].len());

				if old != self.sparse_edge_map[&n][j]
				{
					old = n;
					n = self.sparse_edge_map[&n][j]
				}
			}

			s5 = n;
			s6 = self.flipper.as_ref().unwrap().next(&s5);
			n  = s6;

			for _ in 0..Self::WALK_STEPS
			{
				let j = self.random_generator.gen_range(0..self.sparse_edge_map[&n].len());

				if old != self.sparse_edge_map[&n][j]
				{
					old = n;
					n = self.sparse_edge_map[&n][j]
				}
			}

			s7 = n;
			s8 = self.flipper.as_ref().unwrap().next(&s7);

			if 
			!(
				   s1 == s3 || s1 == s4 || s1 == s5 || s1 == s6 || s1 == s7 || s1 == s8
				|| s2 == s3 || s2 == s4 || s2 == s5 || s2 == s6 || s2 == s7 || s2 == s8
										|| s3 == s5 || s3 == s6 || s3 == s7 || s3 == s8
										|| s4 == s5 || s4 == s6 || s4 == s7 || s4 == s8
																|| s5 == s7 || s5 == s8
																|| s6 == s7 || s6 == s8
			)
			{
				break;
			}
		}

		return (s1, s2, s3, s4, s5, s6, s7, s8);
	}



	/// Gives a starting point for the walk kick
	fn
	first_kicker
	(
		&mut self
	)
	-> (TSPNodeID, TSPNodeID)
	{		
		let mut t1;
		let mut t2;

		let mut try1: TSPNodeID = self.random_generator.gen_range(0..self.tsp_data.n);
		let mut next = self.flipper.as_ref().unwrap().next(&try1);
		let mut prev = self.flipper.as_ref().unwrap().prev(&try1);

		let mut edge_length_to_next = self.tsp_data.get_distance_between_via_id(try1, next);
		let mut edge_length_to_prev = self.tsp_data.get_distance_between_via_id(try1, prev);

		let best;

		if (edge_length_to_next >= edge_length_to_prev)
		{
			t1 = try1;
			t2 = next;
			best = 
				edge_length_to_next 
				- self.tsp_data.get_distance_between_via_id(
					t1, 
					*self.sparse_edge_map[&t1].first().unwrap()
				);
		}
		else
		{
			t1 = prev;
			t2 = try1;
			best = 
				edge_length_to_prev 
				- self.tsp_data.get_distance_between_via_id(
					t1, 
					*self.sparse_edge_map[&t1].first().unwrap()
				);
		}

		for _ in 0..((self.tsp_data.n as f64 * 0.001) as usize +10)
		{
			try1 = self.random_generator.gen_range(0..self.tsp_data.n);
			next = self.flipper.as_ref().unwrap().next(&try1);
			prev = self.flipper.as_ref().unwrap().prev(&try1);
			edge_length_to_next = self.tsp_data.get_distance_between_via_id(try1, next);
			edge_length_to_prev = self.tsp_data.get_distance_between_via_id(try1, prev);

			// Update t1 and t2 in case a better edge has been found
			// Note that CONCORDE does NOT update the value of best at this 
			// point. 
			if (edge_length_to_next >= edge_length_to_prev)
			{
				let len = 
					edge_length_to_next 
					- self.tsp_data.get_distance_between_via_id(
						try1, 
						*self.sparse_edge_map[&try1].first().unwrap()
					);

				if len > best
				{
					t1 = try1;
					t2 = next;
				}
			}
			else
			{
				let len = 
					edge_length_to_prev 
					- self.tsp_data.get_distance_between_via_id(
						try1, 
						*self.sparse_edge_map[&try1].first().unwrap()
					);

				if len > best
				{
					t1 = prev;
					t2 = try1;
				}
			}
		}

		return (t1, t2);
	}
}