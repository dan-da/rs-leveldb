# Rust leveldb bindings

Almost-complete bindings for leveldb for Rust.

# Forked from and change 
This repository forked from `https://github.com/skade/leveldb.git`.  

## Prerequisites

`snappy` and `leveldb` need to be installed. On Ubuntu, I recommend:

```sh
sudo apt-get install libleveldb-dev libsnappy-dev
```

## Usage

If your project is using [Cargo](http://crates.io), drop the following lines in your `Cargo.toml`:

```text
[dependencies]

rs-leveldb = "0.1.3"
```

## Development

Make sure you have all prerequisites installed. Run

```sh
$ cargo build
```

for building and

```sh
$ cargo test
```

to run the test suite.

## Examples

```rust
use leveldb::options::{Options, WriteOptions, ReadOptions};
use leveldb::db::Database;
use leveldb::error::Error;
use leveldb::util::FromU8;
use leveldb::iterator::Iterable;
use std::path::Path;

fn main() -> Result<(), Error> {
    let path = Path::new("temp_ldb");
    let mut options = Options::new();
    options.create_if_missing = true;

    let database = Database::open(&path, &options)?;

    let write_ops = WriteOptions::new();
    let read_ops = ReadOptions::new();

    // key of &[u8] type, it's the world 'name';
    let key = &[110, 97, 109, 101][..];
    database.put(&write_ops, &key, &b"tom"[..])?;

    let value = database.get(&read_ops, &key)?;

    assert!(value.is_some());
    assert_eq!(Vec::from(&b"tom"[..]), value.unwrap());

    // key of &str type
    let key = "age";
    database.put(&write_ops, &key, &b"5"[..])?;

    let value = database.get(&read_ops, &key)?;
    assert!(value.is_some());
    assert_eq!(Vec::from(&b"5"[..]), value.unwrap());

    // key of String type
    let key = "from".to_string();
    database.put(&write_ops, &key, &b"mars"[..])?;

    let value = database.get(&read_ops, &key)?;
    assert!(value.is_some());
    assert_eq!(Vec::from(&b"mars"[..]), value.unwrap());

    // key of integer type
    database.put(&write_ops, &1000, &10000i32.to_be_bytes()[..])?;

    let value = database.get(&read_ops, &1000)?;
    assert!(value.is_some());
    assert_eq!(10000, i32::from_u8(value.unwrap().as_slice()));

    // use put_u8 and get_u8
    let key = &b"temp"[..];
    database.put_u8(&write_ops, key, &b"temp"[..])?;

    let value = database.get_u8(&read_ops, key)?;
    assert!(value.is_some());
    assert_eq!(Vec::from(&b"temp"[..]), value.unwrap());

    // delete use key of integer, &str, String
    database.delete(&write_ops, &1000)?;
    let value = database.get(&read_ops, &1000)?;
    assert!(value.is_none());

    // delete use key of type &[u8]
    database.delete_u8(&write_ops, &b"temp"[..])?;
    let value = database.get(&read_ops, &&b"key"[..])?;
    assert!(value.is_none());

    // iterator
    let iter = database.iter(&read_ops);

    let mut key_and_values = vec![("name", "tom"), ("age", "5"), ("from", "mars")];
    key_and_values.sort();

    for entry in  iter.enumerate() {
        let (i, (key, value)) = entry;
        let key_str = String::from_utf8_lossy(key.as_slice());
        let value_str = String::from_utf8_lossy(value.as_slice());

        let (expected_key, expected_value) = key_and_values.get(i).unwrap();

        assert_eq!(*expected_key, &key_str.to_string());
        assert_eq!(*expected_value, &value_str.to_string());
    }


    Ok(())
}
```

## Open issues

* Filter policies are missing
* Iterators with arbirary start and end points are unsupported

# License

MIT, see `LICENSE`
