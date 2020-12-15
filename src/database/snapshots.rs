//! leveldb snapshots
//!
//! Snapshots give you a reference to the database at a certain
//! point in time and won't change while you work with them.
use leveldb_sys::{leveldb_t, leveldb_snapshot_t};
use leveldb_sys::{leveldb_release_snapshot, leveldb_create_snapshot};

use super::db::Database;
use super::error::Error;
use super::options::ReadOptions;
use super::key::IntoLevelDBKey;
use super::iterator::{Iterable, Iterator, KeyIterator, ValueIterator};

#[allow(missing_docs)]
struct RawSnapshot {
    db_ptr: *mut leveldb_t,
    ptr: *mut leveldb_snapshot_t,
}

impl Drop for RawSnapshot {
    fn drop(&mut self) {
        unsafe { leveldb_release_snapshot(self.db_ptr, self.ptr) };
    }
}

/// A database snapshot
///
/// Represents a database at a certain point in time,
/// and allows for all read operations (get and iteration).
pub struct Snapshot<'a> {
    raw: RawSnapshot,
    database: &'a Database,
}


impl<'a> Snapshot<'a> {
    /// fetches a key from the database
    ///
    /// Inserts this snapshot into ReadOptions before reading

    pub fn get(&'a self,
                  options: &mut ReadOptions<'a>,
                  key: &dyn IntoLevelDBKey)
                  -> Result<Option<Vec<u8>>, Error> {

        options.snapshot = Some(self);
        self.database.get(&options, key)
    }

    pub fn get_u8(&'a self,
               options: &mut ReadOptions<'a>,
               key: &[u8])
               -> Result<Option<Vec<u8>>, Error> {
        options.snapshot = Some(self);
        self.database.get_u8(&options, key)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn raw_ptr(&self) -> *mut leveldb_snapshot_t {
        self.raw.ptr
    }
}

/// Structs implementing the Snapshots trait can be
/// snapshotted.
pub trait Snapshots {
    /// Creates a snapshot and returns a struct
    /// representing it.
    fn snapshot(&self) -> Snapshot;
}

impl Snapshots for Database {
    fn snapshot(&self) -> Snapshot {
        let db_str = self.database.ptr;
        let snap = unsafe {
            leveldb_create_snapshot(db_str)
        };

        let raw = RawSnapshot {
            db_ptr: db_str,
            ptr: snap,
        };

        Snapshot {
            raw,
            database: self
        }
    }
}

impl<'a> Iterable<'a> for Snapshot<'a> {
    fn iter(&'a self, options: &ReadOptions<'a>) -> Iterator<'a> {
        let mut opts = options.clone();
        opts.snapshot = Some(self);

        self.database.iter(&opts)
    }

    fn keys_iter(&'a self, options: &ReadOptions<'a>) -> KeyIterator<'a> {
        let mut opts = options.clone();
        opts.snapshot = Some(self);

        self.database.keys_iter(&opts)
    }

    fn value_iter(&'a self, options: &ReadOptions<'a>) -> ValueIterator<'a> {
        let mut opts = options.clone();
        opts.snapshot = Some(self);

        self.database.value_iter(&opts)
    }
}

