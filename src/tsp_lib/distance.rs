use std::str::FromStr;

/// The different types of distances supported
/// Currently this only allows for 2-dimensional euclidean distances as this
/// one of the more common cases with TSP instances. 
/// Anything 3-dimenional would require adding a third dimension everywhere
/// where the existing 2 dimensions are needed for making decisions, e.g. 
/// building a KD-tree.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum
EDistance
{
	EUCLIDEAN_2D,
}

impl
FromStr
for
EDistance
{
	type Err = ();

	/// Allows the creation of distance enum values from given strings as 
	/// provided for example by TSPLIB input files.
	fn 
	from_str
	(
		s: &str
	) 
	-> Result<Self, Self::Err>
	{
		match s.to_uppercase().as_str()
		{
			"EUC_2D" 	=> Ok(EDistance::EUCLIDEAN_2D),
			_ 			=> Err(())
		}
	}
}