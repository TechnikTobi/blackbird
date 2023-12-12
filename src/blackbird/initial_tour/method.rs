use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum EInitialTourMethod 
{
	Random,
	Boruvka,
	QuickBoruvka,
}

impl
FromStr
for
EInitialTourMethod
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
		match s.to_lowercase().as_str().chars().nth(0).unwrap_or(' ')
		{
			'r'                        => Ok(EInitialTourMethod::Random),
			'b'                        => Ok(EInitialTourMethod::Boruvka),
			'q'                        => Ok(EInitialTourMethod::QuickBoruvka),
			_                          => Err(())
		}
	}
}

