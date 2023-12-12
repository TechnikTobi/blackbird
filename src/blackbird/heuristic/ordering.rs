use crate::{blackbird::data::{bb_data::BBData, bb_edge::BBEdge}, tsp_lib::node::{TSPNodeID, TSPWeight}};

use super::breadth::breadth;

impl
BBData
{
	/// Based on the CONCORDE function 'look_ahead', defined in 'linkern.c'
	pub fn
	lk_ordering
	(
		&self,
		first:                         TSPNodeID,
		last:                          TSPNodeID,
		level:                         usize,
		gain:                          TSPWeight,
		
	)
	-> Vec<(BBEdge, TSPWeight)>
	{
		let mut ordering = Vec::new();

		for this in &self.sparse_edge_map[&last]
		{
			let edge = BBEdge::new(&self.tsp_data, last, *this);

			if edge.weight > gain
			{
				break;
			}

			if 
			(
				   !self.is_edge_deleted(&edge)
				&& this != &first
				&& this != &self.flipper.as_ref().unwrap().next(&last)
			)
			{
				let prev = self.flipper.as_ref().unwrap().prev(this);
				let other_edge = BBEdge::new(&self.tsp_data, *this, prev);

				if !self.is_edge_added(&other_edge)
				{
					let value = edge.weight - other_edge.weight;
					ordering.push((other_edge, value));	
				}
			}
		}

		// Sort by value 'val'
		ordering.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap());

		// Return only as many as the breadth as this level allows it
		if ordering.len() > breadth(level)
		{
			return ordering[..breadth(level)].to_vec();
		}
		else
		{
			return ordering;
		}
	}

	pub fn
	lk_ordering_no_backtracking
	(
		&self,
		first:                         TSPNodeID,
		last:                          TSPNodeID,
	)
	-> Option<(BBEdge, bool, TSPWeight)>
	{
		let mut winner: Option<BBEdge> = None;
		let mut mak_morton_edge = false;
		let mut diff = TSPWeight::MAX;

		for this in &self.sparse_edge_map[&last]
		{
			let edge = BBEdge::new(&self.tsp_data, *this, last);

			if
			(
				   !self.is_edge_deleted(&edge)
				&& this != &first
				&& this != &self.flipper.as_ref().unwrap().next(&last)
			)
			{
				let prev = self.flipper.as_ref().unwrap().prev(this);
				let prev_edge = BBEdge::new(&self.tsp_data, *this, prev);

				if (!self.is_edge_added(&prev_edge))
				{
					let value = edge.weight - prev_edge.weight;

					if value < diff
					{
						diff = value;
						winner = Some(prev_edge);	
					}
				}
			}
		}

		let first_prev = self.flipper.as_ref().unwrap().prev(&first);

		for this in &self.sparse_edge_map[&first]
		{
			let edge = BBEdge::new(&self.tsp_data, *this, first);

			if
			(
				   !self.is_edge_deleted(&edge)
				&& this != &first
				&& this != &first_prev
			)
			{
				let next = self.flipper.as_ref().unwrap().next(this);
				let next_edge = BBEdge::new(&self.tsp_data, *this, next);

				if (!self.is_edge_added(&next_edge))
				{
					let value = edge.weight - next_edge.weight;

					if value < diff
					{
						winner = Some(next_edge);	
						mak_morton_edge = true;
						diff = value;
					}
				}
			}
		}

		if let Some(winner_edge) = winner
		{
			return Some((winner_edge, mak_morton_edge, diff));
		}
		
		return None;
	}



	const ALTERNATE_LOOK_AHEAD_MAX_1: usize = 4;
	const ALTERNATE_LOOK_AHEAD_MAX_2: usize = 3;
	const ALTERNATE_LOOK_AHEAD_MAX_3: usize = 3;



	pub fn
	alternate_look_ahead_1
	(
		&self,
		gain:                          TSPWeight,
		t1:                            TSPNodeID,
		t2:                            TSPNodeID,
	)
	-> Vec<(BBEdge, TSPWeight)>
	{
		let mut ordering = Vec::new();

		for this in &self.sparse_edge_map[&t2]
		{
			if this == &t1
			{
				continue;
			}

			let t2_this_edge = BBEdge::new(&self.tsp_data, t2, *this);

			if t2_this_edge.weight > gain
			{
				break;
			}

			let next = self.flipper.as_ref().unwrap().next(this);
			let edge = BBEdge::new(&self.tsp_data, *this, next);
			let val  = t2_this_edge.weight - edge.weight;

			ordering.push((edge, val));
		}

		// Sort by value 'val'
		ordering.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap());

		// Return only as many as the breadth as allowed for this look ahead type
		if ordering.len() > Self::ALTERNATE_LOOK_AHEAD_MAX_1
		{
			return ordering[..Self::ALTERNATE_LOOK_AHEAD_MAX_1].to_vec();
		}
		else
		{
			return ordering;
		}
	}

	pub fn
	alternate_look_ahead_2
	(
		&self,
		gain:                          TSPWeight,
		t2:                            TSPNodeID,
		t3:                            TSPNodeID,
		t4:                            TSPNodeID,
	)
	-> Vec<(BBEdge, TSPWeight, bool, bool)>
	{
		// Vector of tuples, consisting of original CONCORDE values
		// - The edge itself
		// - The values 'val' 'seq' and 'side'
		let mut ordering = Vec::new();

		for t5 in &self.sparse_edge_map[&t4]
		{
			if self.weirdmark.get(t5).unwrap_or(&0) != &self.weirdmagic
			{
				let t4_t5_edge = BBEdge::new(&self.tsp_data, t4, *t5);

				if t4_t5_edge.weight > gain
				{
					break;
				}

				let mut t6 = self.flipper.as_ref().unwrap().prev(t5);

				if t2 == t6 || t3 == t6 { continue; }

				let mut t5_t6_edge = BBEdge::new(&self.tsp_data, *t5, t6);
				let mut val        = t4_t5_edge.weight - t5_t6_edge.weight;
				let     seq        = self.flipper.as_ref().unwrap().sequence(&t2, t5, &t3);
				ordering.push((t5_t6_edge, val, seq, false));

				if self.flipper.as_ref().unwrap().sequence(&t2, t5, &t3)
				{
					t6 = self.flipper.as_ref().unwrap().next(t5);

					if t2 == t6 || t3 == t6 { continue; }

					t5_t6_edge = BBEdge::new(&self.tsp_data, *t5, t6);
					val        = t4_t5_edge.weight - t5_t6_edge.weight;
					ordering.push((t5_t6_edge, val, seq, true));
				}
			}
		}

		// Sort by value 'val'
		ordering.sort_by(|(_, a, _, _), (_, b, _, _)| a.partial_cmp(&b).unwrap());

		// Return only as many as the breadth as allowed for this look ahead type
		if ordering.len() > Self::ALTERNATE_LOOK_AHEAD_MAX_2
		{
			return ordering[..Self::ALTERNATE_LOOK_AHEAD_MAX_2].to_vec();
		}
		else
		{
			return ordering;
		}
	}

	pub fn
	alternate_look_ahead_3
	(
		&self,
		gain:                          TSPWeight,
		t2:                            TSPNodeID,
		t3:                            TSPNodeID,
		t6:                            TSPNodeID,
	)
	-> Vec<(BBEdge, TSPWeight, bool)>
	{
		// Vector of tuples, consisting of original CONCORDE values
		// - The edge itself
		// - The values 'val' and 'side'
		let mut ordering = Vec::new();

		for t7 in &self.sparse_edge_map[&t6]
		{
			let t6_t7_edge = BBEdge::new(&self.tsp_data, t6, *t7);

			if t6_t7_edge.weight > gain
			{
				break;
			}

			if 
			(
				   self.weirdmark.get(t7).unwrap_or(&0) != &self.weirdmagic
				&& self.flipper.as_ref().unwrap().sequence(&t2, t7, &t3)
			)
			{
				let t8_prev         = self.flipper.as_ref().unwrap().prev(t7);

				if t2 != t8_prev && t3 != t8_prev
				{
					let t7_t8_prev_edge = BBEdge::new(&self.tsp_data, *t7, t8_prev);
					let val_prev        = t6_t7_edge.weight - t7_t8_prev_edge.weight;
					ordering.push((t7_t8_prev_edge, val_prev, false));
				}


				let t8_next         = self.flipper.as_ref().unwrap().next(t7);

				if t2 != t8_next && t3 != t8_next
				{
					let t7_t8_next_edge = BBEdge::new(&self.tsp_data, *t7, t8_next);
					let val_next        = t6_t7_edge.weight - t7_t8_next_edge.weight;
					ordering.push((t7_t8_next_edge, val_next, true));
				}
			}
		}

		// Sort by value 'val'
		ordering.sort_by(|(_, a, _), (_, b, _)| a.partial_cmp(&b).unwrap());

		// Return only as many as the breadth as allowed for this look ahead type
		if ordering.len() > Self::ALTERNATE_LOOK_AHEAD_MAX_3
		{
			return ordering[..Self::ALTERNATE_LOOK_AHEAD_MAX_3].to_vec();
		}
		else
		{
			return ordering;
		}
	}
}