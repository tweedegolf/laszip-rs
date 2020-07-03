#![allow(missing_docs)]
//! Provides bindings to LASzip, the open-source LiDAR compressor, as well as wrappers to read and write both files and memory.
//!
//!
//! ## File example
//!  ```
//! # use laszip::*;
//! #
//! # fn main() -> Result<(), laszip::error::LaszipError> {
//! #    
//! laszip::load_laszip_library();
//!
//! let reader = laszip::LazReader::from_file("../data/building.laz").unwrap();
//! let (scale, offset) = (reader.get_header()?.scale(), reader.get_header()?.offset());
//! let mut writer =
//!     laszip::LazWriter::from_file("../data/out/roof.laz", true, &scale, &offset).unwrap();
//!
//! for p in reader.iter_points()?.filter(|p| {
//!     if let Ok(p) = p {
//!         let z = p.Z as f64 * scale.z + offset.z;
//!         z > 6.0
//!     } else {
//!         false
//!     }
//! }) {
//!     writer.push(p.unwrap())?;
//! }
//! #   
//! #   assert_eq!(1150, writer.get_number_of_points_written());
//! #   
//! #   Ok::<(), laszip::error::LaszipError>(())
//! # }
//! ```

pub mod error;
pub mod geo;
pub mod lafi;
mod laheader;
pub mod lapi;
mod lapoint;
mod lareader;
mod lautil;
mod lawriter;

pub use laheader::*;
pub use lapoint::*;
pub use lareader::*;
pub use lautil::*;
pub use lawriter::*;
