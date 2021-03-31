// Assure truth
pub mod assure;

// Assure value comparison
pub mod assure_eq; // equal
pub mod assure_ne; // not equal
pub mod assure_lt; // less than
pub mod assure_le; // less than or equal to
pub mod assure_gt; // greater than
pub mod assure_ge; // greater than or equal to

// Assure iterator-related set-based comparison
pub mod assure_set_eq; // equal
pub mod assure_set_ne; // not equal

// Assure IO-related truth, which can return Err(std:io:Error(…))
pub mod assure_io;

// Assure IO-related comparison, which can return Err(std:io:Error(…))
pub mod assure_io_eq; // equal
