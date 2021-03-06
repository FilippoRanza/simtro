pub mod fast_line_factory;
mod line;
pub mod line_factory;
pub use line::{Line, LineDirection, SegmentType};

type Duration = usize;
type StationID = usize;
