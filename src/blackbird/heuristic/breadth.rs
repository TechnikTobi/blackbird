pub fn
breadth
(
	k:                             usize
)
-> usize
{
	// According to this source:
	// "The Traveling Salesman Problem: A Computational Study" by
	// - David L. Applegate
	// - Robert E. Bixby
	// - Vašek Chvatál
	// - William J. Cook
	// ISBN: 9780691129938
	// Page 452
	// The breadth is set to 0 for levels deeper than 25
	// Furthermore, for the first values, see "backtrack_count" in "linkern.c"
	match k
	{
		0  => 4,
		1  => 3,
		2  => 3,
		3  => 2,
		4  => 1,
		5  => 1,
		6  => 1,
		7  => 1,
		8  => 1,
		9  => 1,
		10 => 1,
		11 => 1,
		12 => 1,
		13 => 1,
		14 => 1,
		15 => 1,
		16 => 1,
		17 => 1,
		18 => 1,
		19 => 1,
		20 => 1,
		21 => 1,
		22 => 1,
		23 => 1,
		24 => 1,
		_ => 0
	}
}

