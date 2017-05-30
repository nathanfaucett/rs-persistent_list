#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate collection_traits;


mod persistent_list;


pub use self::persistent_list::PersistentList;
