use rand::Rng;


/// This struct represents a single die.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Die {
	/// The result from initialization.
	pub result: i32,
}//end struct Die

impl Die {
	/// Creates a new die with a random result between 1 and 6.
	pub fn new() -> Die {
		Die {
			result: rand::thread_rng().gen_range(1..=6),
		}//end struct construction
	}//end new()
}//end impl for Die
