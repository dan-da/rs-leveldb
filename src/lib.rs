#![crate_type = "lib"]
#![crate_name = "leveldb"]

pub mod database;

pub use database::options;
pub use database::error;
pub use database::snapshots;
pub use database::db;
pub use database::cache;
pub use database::management;
pub use database::iterator;
pub use database::batch;
pub use database::compaction;
pub use database::comparator;
pub use database::key;
pub use database::util;

// re-export any dep crates that are exposed in our public API
pub use leveldb_sys;

use leveldb_sys::{leveldb_major_version, leveldb_minor_version};


/// Library version information
///
/// Need a recent version of leveldb to be used.

pub trait Version {
    /// The major version.
    fn major() -> isize {
        unsafe { leveldb_major_version() as isize }
    }

    /// The minor version
    fn minor() -> isize {
        unsafe { leveldb_minor_version() as isize }
    }
}
