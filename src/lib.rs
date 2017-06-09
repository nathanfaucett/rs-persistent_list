#![feature(alloc)]


extern crate alloc;
extern crate core;

extern crate collection_traits;


mod persistent_list;


pub use self::persistent_list::PersistentList;
