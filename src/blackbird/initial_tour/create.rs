use std::time::Instant;

use crate::blackbird::data::bb_data::*;
use crate::warn;

use super::method::EInitialTourMethod;

impl
BBData
{
	pub fn
	create_initial_tour
	(
		&mut self
	)
	{
		if self.current_tour.is_some()
		{
			warn!("There already is an initial tour that will be overwritten!")
		}

		let time_measurement_start = Instant::now();

		match &self.init_method
		{
			EInitialTourMethod::Random           => self.create_initial_tour_random(),
			EInitialTourMethod::Boruvka          => self.create_initial_tour_boruvka(),
			EInitialTourMethod::QuickBoruvka     => self.create_initial_tour_quick_boruvka(),	
		}

		// Validate the tour
		if let Some(tour_cyle) = self.current_tour.as_ref().unwrap().is_valid(false)
		{
			self.initial_tour_cycle  = Some(tour_cyle);
			self.current_tour_length = self.current_tour.as_ref().unwrap().compute_len();
			self.output_tour_cycle  = self.initial_tour_cycle.clone();
			self.output_tour_length = self.current_tour_length;
			println!("Initial tour length: {}", self.output_tour_length);
			println!("Initial tour creation runtime : {}", (time_measurement_start.elapsed().as_micros() as f64) / 1000000.0);
			
			return;
		}

		panic!("Could not create a valid tour for the given init method!");
	}
}