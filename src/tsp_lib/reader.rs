use std::fs::OpenOptions;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;

use crate::tsp_lib::node::*;
use crate::tsp_lib::distance::*;
use crate::tsp_lib::data::*;

fn 
is_node_coord_line
(
	line: &str
) 
-> bool 
{
	for character in line.chars() 
	{
		if 
			!matches!(character, '0'..='9' | '.' | 'e' | 'E' | '-' | '+' ) && 
			!character.is_whitespace() 
		{
			return false;
		}
	}
	return true;
}

/// Reads in a file in TSPLIB format 
/// For further details, see here: 
/// http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp95.pdf
/// Please note that not all of the features that are defined for TSPLIB are
/// supported by this reader. 
/// For example, distances other than 2-dimensional euclidean or any
/// 3-dimensional data is not suppored at the current stage due to the already
/// complex nature of this project.
/// The result of this is a TSPData struct that contains the necessary
/// information for constructing a TSP instance, like node data. 
pub fn
read_tsplib_file
(
	path_string: &String
)
-> TSPData
{

	// Parameter checking
	if path_string == ""
	{
		panic!("Empty input file path String!");
	}

	// Get the path to the specified file
	let path = Path::new(path_string);

	// Check that the specified file actually exists
	if !path.exists()
	{
		panic!("Non-existant input file!");
	}

	// Holds the value of key "DIMENSION" (if given)
	let mut dimension = 0usize;

	// The TSPData that will eventually get returned
	let mut tsp_data = TSPData::empty(dimension);

	// The input .tsp file and its reader
	let file = OpenOptions::new()
		.write(false)
		.read(true)
		.open(path)
		.expect("Could not open .tsp file");
	let reader = BufReader::new(file);

	// Go through every line
	for (line_nr, optional_line) in reader.lines().enumerate()
	{
		// If the
		if optional_line.is_err()
		{
			panic!("TSPLIB Reader: Could not read line {}", line_nr);
		}

		// Unwrap the line, replace colons and trim whitespace
		let untrimmed_line = optional_line.unwrap();
		let replaced_colons = untrimmed_line.replace(":", " ");
		let line = replaced_colons.trim();

		// Skip empty lines
		if line.len() == 0
		{
			continue;
		}

		// Divide the line into its parts that are separated by whitespaces
		let mut parts = line.split_whitespace();

		// Next, go through the different keys that might be contained

		// Ignore the name of the TSP instance
		if parts.next().unwrap_or_default().starts_with("NAME")
		{}

		else if line.starts_with("TYPE") 
		{
			if !parts.next().unwrap_or_default().eq("TSP")
			{
				panic!("TSPLIB Reader: Not a TSP File!");
			}
		}

		// Ignore any comments as they are not required for the actual TSP
		else if line.starts_with("COMMENT")
		{}

		// Get the number of dimensions 
		else if line.starts_with("DIMENSION")
		{
			dimension = parts.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
		}

		// Get the distance measure to use for solving the TSP
		else if line.starts_with("EDGE_WEIGHT_TYPE")
		{
			if let Ok(distance) = EDistance::from_str(parts.next().unwrap_or_default())
			{
				tsp_data.set_distance_metric(distance);
			}
			else
			{
				panic!("TSPLIB Reader: Illegal EDGE_WEIGHT_TYPE value!");
			}
		}

		// Check if unsupported weight format
		else if line.starts_with("EDGE_WEIGHT_FORMAT")
		{
			panic!("TSPLIB Reader: EDGE_WEIGHT_FORMAT currently not supported!");
		}
		
		// Start of section that contains actual node data
		else if line.starts_with("NODE_COORD_SECTION")
		{
			if tsp_data.node_count() > 0
			{
				panic!("TSPLIB Reader: A second NODE_COORD_SECTION!");
			}
		}
		
		// Explicit edge weights are also not supported
		else if line.starts_with("EDGE_WEIGHT_SECTION")
		{
			panic!("TSPLIB Reader: EDGE_WEIGHT_SECTION currently not supported!");
		}
		
		// Fixed edges section: Also not supported
		else if line.starts_with("FIXED_EDGES_SECTION")
		{
			panic!("TSPLIB Reader: FIXED_EDGES_SECTION not supported!");
		}

		// The end of the input file
		else if line.starts_with("EOF")
		{
			break;
		}
		
		// Try to parse the line as node data
		else if is_node_coord_line(line)
		{
			// Construct node struct
			let mut node_parts = line.split_whitespace();

			// Don't use numbering from the TSP but own "internal" numbering
			let unused_node_id = node_parts.next().to_owned().unwrap_or("0").parse::<usize>().unwrap_or(0);

			let node = TSPNode::new(
				tsp_data.node_count(),
				node_parts.next().to_owned().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
				node_parts.next().to_owned().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
			);

			if unused_node_id == 0
			{
				panic!("TSPLIB Reader: Could not read node!");
			}
			
			tsp_data.add_node(&node);
		}
		
		// In any other case: I have no idea what this line says
		else
		{
			panic!("TSPLIB Reader: Unknown line type: {}", line);
		}
	}

	// Check that added nodes corresponds with the given dimension count
	if tsp_data.node_count() != dimension
	{
		panic!("TSPLIB Reader: Dimension and node count do not match");
	}

	// tsp_data.print();
	return tsp_data;
}