use std::collections::VecDeque;
use std::time::Instant;

use rand::seq::SliceRandom;

use crate::blackbird::data::bb_data::*;
use crate::blackbird::data::bb_flipper::BBFlipper;
use crate::blackbird::data::bb_edge::BBEdge;
use crate::tsp_lib::node::TSPNodeID;
use crate::tsp_lib::node::TSPWeight;

impl
BBData
{
	/// The main function for handling the call to the heuristic
	/// This performs
	/// - Time measurement
	/// - Initial tour creation
	/// - Computing of the sparse edge map
	/// - Performing one or multiple runs of CLK
	/// - Printing the final results
	pub fn
	main_heuristic
	(
		&mut self
	)
	{
		// Start time measurement
		let time_measurement_start = Instant::now();

		// Create an initial tour with the selected algorithm
		self.create_initial_tour();

		// Compute the sparse set of good edges to consider during CLK
		self.sparse_edge_map();

		// Apply CLK as often as specified (or just once)
		for _ in 0..(if (self.cli_args.number_of_runs > 0) { self.cli_args.number_of_runs } else { 1 })
		{
			// Call the CLK heuristic
			self.chained_lin_kernighan();
		}

		// Total runtime
		if (self.cli_args.verbose)
		{
			println!("Total runtime : {}µs", time_measurement_start.elapsed().as_micros());
		}

		// The final result
		println!("Final tour length : {}", self.output_tour_length);
		println!("Total runtime : {}", (time_measurement_start.elapsed().as_micros() as f64) / 1000000.0);
	}

	/// The heart of the heuristic which chains multiple calls to LK and the
	/// kick method together to form CLK. This performs
	/// - Time measurement
	/// - Creation of the Flipper
	/// - Creation of the queue for the nodes to be processed
	/// - A first call to LK
	/// - A loop of kicking and calling LK
	fn
	chained_lin_kernighan
	(
		&mut self
	)
	{
		// Start time measurement of this round of CLK
		let time_measurement_start = Instant::now();

		// Construct the flipper for storing the current tour and handling flips
		self.flipper = Some(BBFlipper::new(&self.initial_tour_cycle.as_ref().unwrap()));

		// Initialize the node queue with random order
		let mut shuffled_node_ids = self.tsp_data.nodes.iter().map(|node| node.id).collect::<Vec<TSPNodeID>>();
		shuffled_node_ids.shuffle(&mut self.random_generator);
		self.node_queue = VecDeque::from(shuffled_node_ids);

		// Clear the edge markings
		self.edge_markings.clear();

		for edge in &self.current_tour.as_ref().unwrap().edges.clone()
		{
			self.mark_edge_as_added(&edge);
		}

		// Call Lin Kernighan for the first time before we can kick the tour
		self.lin_kernighan();

		let number_of_kicks = self.tsp_data.n;
		let stall_count = 10000000;

		let mut quitcount = std::cmp::min(stall_count, number_of_kicks);
		let mut round = 0;

		while round < quitcount
		{
			self.current_tour_length = self.flipper.as_ref().unwrap().cost(&self.tsp_data);
	
			self.kick();
	
			if self.lin_kernighan()
			{
				quitcount = std::cmp::min(round + stall_count, number_of_kicks);
			}

			// Stop if time bound is reached
			if (self.cli_args.time_limit <= time_measurement_start.elapsed().as_secs())
			{
				break;
			}

			// Stop if length bound is reached
			if (self.cli_args.length_limit >= self.output_tour_length)
			{
				break;
			}

			// Increase round counter
			round += quitcount;
		}

		println!("CLK runtime : {}", (time_measurement_start.elapsed().as_micros() as f64) / 1000000.0);
	}



	/// The main Lin Kernighan function that performs a single run of the LK heuristic
	fn
	lin_kernighan
	(
		&mut self
	)
	-> bool
	{
		while let Some(start) = self.node_queue.pop_front()
		{
			self.improve(start);
		}

		let output_tour_cycle = self.flipper.as_ref().unwrap().as_cycle();
		let mut length = 0.0;
		for i in 0..output_tour_cycle.len()
		{
			length += self.tsp_data.get_distance_between_via_id(output_tour_cycle[i], output_tour_cycle[(i+1)%self.tsp_data.n]);
		}

		if length < self.output_tour_length
		{
			self.output_tour_cycle = Some(output_tour_cycle);
			self.output_tour_length = length;
			return true;
		}

		return false;
	}

	const EPSILON: TSPWeight = 0.00000001;

	fn
	improve
	(
		&mut self,
		base:                          TSPNodeID
	)
	-> TSPWeight
	{
		let base_next = self.flipper.as_ref().unwrap().next(&base);
		let edge = BBEdge::new(&self.tsp_data, base, base_next);
		let gain = edge.weight;

		self.mark_edge_as_deleted(&edge);

		let (hit, mut g_star) = self.step(base, base_next, 0, gain, 0.0);

		if hit == 0
		{
			g_star = self.alternate_step(base, base_next, gain)
		}

		self.unmark_edge_as_deleted(&edge);

		if g_star > Self::EPSILON
		{
			self.add_to_queue(base);
			self.add_to_queue(base_next);
		}

		return g_star;
	}

	const BACKTRACK: usize = 4;
	const MAX_DEPTH: usize = 25;

	// Implements the step function as described in 
	// "The Traveling Salesman Problem: A Computational Study" by
	// - David L. Applegate
	// - Robert E. Bixby
	// - Vašek Chvatál
	// - William J. Cook
	// ISBN: 9780691129938
	// Page 430
	// TODO: FIND BETTER VARIABLE NAMES
	fn
	step
	(
		&mut self,
		first:                         TSPNodeID,
		last:                          TSPNodeID,
		level:                         usize,
		old_gain:                      TSPWeight,
		old_g_star:                    TSPWeight,
	)
	-> (usize, TSPWeight)
	{
		let mut hits = 0;
		let mut g_star = old_g_star;

		if (level >= Self::BACKTRACK)
		{
			return self.step_no_backtracking(first, last, level, old_gain, old_g_star)
		}

		// Go through all the edges that were obtained using the lk ordering
		for (edge, diff) in &self.lk_ordering(first, last, level, old_gain)
		{
			// "Disassembling" the edge from the ordering
			let this     = edge.start;
			let new_last = edge.end;

			// Compute the change in tour length
			let gain = old_gain - diff;
			let val  = gain - self.tsp_data.get_distance_between_via_id(new_last, first);

			if val > g_star
			{
				g_star = val;
				hits += 1;
			}

			self.flipper.as_mut().unwrap().flip(last, new_last);

			if (level < Self::MAX_DEPTH)
			{
				let added_edge   = BBEdge::new_weightless(last, this);
				let deleted_edge = edge.clone();

				self.mark_edge_as_added(&added_edge);
				self.mark_edge_as_deleted(&deleted_edge);

				let (hits_result, g_star_result) = self.step(first, new_last, level+1, gain, g_star);
				hits += hits_result;
				g_star = g_star_result;

				self.unmark_edge_as_added(&added_edge);
				self.unmark_edge_as_deleted(&deleted_edge);
			}

			if hits > 0
			{
				self.add_to_queue(this);
				self.add_to_queue(new_last);
				return (1, g_star);
			}
			else
			{
				self.flipper.as_mut().unwrap().unflip(last, new_last);
			}
		}

		return (0, g_star);
	}



	/// The step function that explicitly does not use backtracking but 
	/// provides support for Mak-Morton moves
	fn
	step_no_backtracking
	(
		&mut self,
		first:                         TSPNodeID,
		last:                          TSPNodeID,
		level:                         usize,
		old_gain:                      TSPWeight,
		old_g_star:                    TSPWeight,
	)
	-> (usize, TSPWeight)
	{
		let mut g_star = old_g_star;

		if let Some((edge, is_mak_morton_edge, diff)) = self.lk_ordering_no_backtracking(first, last)
		{
			let mut hit = 0;
			let this = edge.start;
			let new_other = edge.end;

			let gain = old_gain - diff;
			let val = gain - self.tsp_data.get_distance_between_via_id(new_other, if is_mak_morton_edge { last } else { first } );
			if val > g_star
			{
				g_star = val;
				hit += 1;
			}

			let added_edge;
			let deleted_edge;

			if is_mak_morton_edge
			{
				self.flipper.as_mut().unwrap().flip(new_other, first);
			}
			else
			{
				self.flipper.as_mut().unwrap().flip(last, new_other);
			}

			if (level < Self::MAX_DEPTH)
			{
				if is_mak_morton_edge
				{
					added_edge   = BBEdge::new_weightless(first, this);
					deleted_edge = BBEdge::new_weightless(this, new_other);
				}
				else
				{
					added_edge   = BBEdge::new_weightless(last, this);
					deleted_edge = BBEdge::new_weightless(this, new_other);
				}

				self.mark_edge_as_added(&added_edge);
				self.mark_edge_as_deleted(&deleted_edge);

				let (hit_result, g_star_result) = if is_mak_morton_edge
				{
					self.step_no_backtracking(new_other, last, level+1, gain, g_star)
				}
				else
				{
					self.step_no_backtracking(first, new_other, level+1, gain, g_star)
				};
				hit += hit_result;

				assert!(g_star_result >= g_star);

				g_star = g_star_result;

				self.unmark_edge_as_added(&added_edge);
				self.unmark_edge_as_deleted(&deleted_edge);
			}

			if (hit > 0) 
			{
				self.add_to_queue(this);
				self.add_to_queue(new_other);

				return (1, g_star);
			}
			else
			{
				if is_mak_morton_edge
				{
					self.flipper.as_mut().unwrap().unflip(new_other, first);
				}
				else
				{
					self.flipper.as_mut().unwrap().unflip(last, new_other);
				}
			}	
		}

		return (0, g_star);
	}



	/// Add a given TSPNodeID to the queue of nodes to be processed in the future
	fn
	add_to_queue
	(
		&mut self,
		node_id:                       TSPNodeID,
	)
	{
		if self.node_queue.contains(&node_id)
		{
			return;
		}
		self.node_queue.push_back(node_id);
	}



	/// The weird_second_step that serves as an alternative recursion for step
	/// in case the 'normal' step method fails to provide an improvement
	fn
	alternate_step
	(
		&mut self,
		t1:                            TSPNodeID,
		t2:                            TSPNodeID,
		gain:                          TSPWeight,
	)
	-> TSPWeight
	{
		let mut hit;
		let mut g_star = 0.0;

		for (edge1, diff1) in self.alternate_look_ahead_1(gain, t1, t2)
		{
			let t3 = edge1.start;
			let t4 = edge1.end;

			let old_gain = gain - diff1;

			let t4_next = self.flipper.as_ref().unwrap().next(&t4);

			let added_edge1   = BBEdge::new_weightless(t2, t3);
			let deleted_edge1 = BBEdge::new_weightless(t3, t4);

			self.mark_edge_as_added(&added_edge1);
			self.mark_edge_as_deleted(&deleted_edge1);

			self.weirdmagic += 1;
			self.weirdmark.insert(t1,      self.weirdmagic);
			self.weirdmark.insert(t2,      self.weirdmagic);
			self.weirdmark.insert(t3,      self.weirdmagic);
			self.weirdmark.insert(t4_next, self.weirdmagic);

			for (edge2, diff2, seq2, side2) in self.alternate_look_ahead_2(old_gain, t2, t3, t4)
			{
				let t5 = edge2.start;
				let t6 = edge2.end;

				let added_edge2 = BBEdge::new_weightless(t4, t5);

				self.mark_edge_as_added(&added_edge2);


				if seq2
				{
					let gain = old_gain - diff2;
					let val  = gain - self.tsp_data.get_distance_between_via_id(t6, t1);

					if val > g_star
					{
						g_star = val;
					}

					if !side2
					{
						self.flipper.as_mut().unwrap().flip(t2, t6);
						self.flipper.as_mut().unwrap().flip(t5, t3);
					}
					else
					{
						self.flipper.as_mut().unwrap().flip(t2, t3);
						self.flipper.as_mut().unwrap().flip(t5, t2);
						self.flipper.as_mut().unwrap().flip(t3, t6);
					}


					let deleted_edge2 = BBEdge::new_weightless(t5, t6);

					self.mark_edge_as_deleted(&deleted_edge2);
					(hit, g_star) = self.step(t1, t6, 2, gain, g_star);
					self.unmark_edge_as_deleted(&deleted_edge2);
					
					if hit == 0 && g_star > 0.0
					{
						hit = 1;
					}

					if hit == 0
					{
						if !side2
						{
							self.flipper.as_mut().unwrap().unflip(t5, t3);
							self.flipper.as_mut().unwrap().unflip(t2, t6);
						}
						else
						{
							self.flipper.as_mut().unwrap().unflip(t3, t6);
							self.flipper.as_mut().unwrap().unflip(t5, t2);
							self.flipper.as_mut().unwrap().unflip(t2, t3);	
						}
					}
					else
					{
						self.unmark_edge_as_added(&added_edge2);
						self.unmark_edge_as_added(&added_edge1);
						self.unmark_edge_as_deleted(&deleted_edge1);
						self.add_to_queue(t3);
						self.add_to_queue(t4);
						self.add_to_queue(t5);
						self.add_to_queue(t6);

						// println!("Alternate Step! Return 3");
						return g_star;
					}
				}
				else
				{
					let t_g = old_gain - diff2;
					let deleted_edge2 = BBEdge::new_weightless(t5, t6);
					self.mark_edge_as_deleted(&deleted_edge2);

					for (edge3, diff3, side3) in self.alternate_look_ahead_3(t_g, t2, t3, t6)
					{
						// println!("Alternate Step! (4)");
						let t7 = edge3.start;
						let t8 = edge3.end;

						let gain = t_g - diff3;
						let val  = gain - self.tsp_data.get_distance_between_via_id(t8, t1);

						if val > g_star
						{
							g_star = val;
						}

						if !side3
						{
							self.flipper.as_mut().unwrap().flip(t2, t8);
							self.flipper.as_mut().unwrap().flip(t7, t3);
							self.flipper.as_mut().unwrap().flip(t4, t6);
						}
						else
						{
							self.flipper.as_mut().unwrap().flip(t2, t6);
							self.flipper.as_mut().unwrap().flip(t6, t8);
							self.flipper.as_mut().unwrap().flip(t4, t2);
						}

						let added_edge3   = BBEdge::new_weightless(t6, t7);
						let deleted_edge3 = BBEdge::new_weightless(t7, t8);

						self.mark_edge_as_added(&added_edge3);
						self.mark_edge_as_deleted(&deleted_edge3);

						(hit, g_star) = self.step(t1, t8, 3, gain, g_star);

						self.unmark_edge_as_added(&added_edge3);
						self.unmark_edge_as_deleted(&deleted_edge3);

						if hit == 0 && g_star > 0.0
						{
							hit = 1;
						}

						if hit == 0
						{
							if !side3
							{
								self.flipper.as_mut().unwrap().unflip(t4, t6);
								self.flipper.as_mut().unwrap().unflip(t7, t3);
								self.flipper.as_mut().unwrap().unflip(t2, t8);
							}
							else
							{
								self.flipper.as_mut().unwrap().unflip(t4, t2);
								self.flipper.as_mut().unwrap().unflip(t6, t8);
								self.flipper.as_mut().unwrap().unflip(t2, t6);
							}
						}
						else
						{
							self.unmark_edge_as_added(&added_edge2);
							self.unmark_edge_as_added(&added_edge1);
							self.unmark_edge_as_deleted(&deleted_edge2);
							self.unmark_edge_as_deleted(&deleted_edge1);

							self.add_to_queue(t3);
							self.add_to_queue(t4);
							self.add_to_queue(t5);
							self.add_to_queue(t6);
							self.add_to_queue(t7);
							self.add_to_queue(t8);

							return g_star;
						}
					}

					self.unmark_edge_as_deleted(&deleted_edge2)
				}

				self.unmark_edge_as_added(&added_edge2);
			}

			self.unmark_edge_as_added(&added_edge1);
			self.unmark_edge_as_deleted(&deleted_edge1);
		}
		return 0.0;
	}
}