use crate::tsp_lib::node::TSPWeight;

/// A macro for printing warnings to the user
#[macro_export]
macro_rules! warn {
	($warning_message:expr) 
	=>
	{
		println!("WARNING: {}", $warning_message)
	}
}

/// A simple function for getting the minimum of two TSPWeight values
pub fn
tsp_weight_min
(
	a:                                 TSPWeight,
	b:                                 TSPWeight,
)
-> TSPWeight
{
	if a < b { a } else { b }
}

/// A simple function for getting the maximum of two TSPWeight values
pub fn
tsp_weight_max
(
	a:                                 TSPWeight,
	b:                                 TSPWeight,
)
-> TSPWeight
{
	if a > b { a } else { b }
}