#![feature(test)]


extern crate test;

extern crate persistent_list;
extern crate linked_list;
extern crate collection_traits;


use test::Bencher;

use collection_traits::*;


const SIZE: usize = 1024;


#[bench]
fn bench_persistent_list(b: &mut Bencher) {
    use persistent_list::PersistentList;

    b.iter(|| {
        let mut a = PersistentList::new();

        for i in 0..SIZE {
            a = a.push(i);
        }

        a
    });
}
#[bench]
fn bench_linked_list(b: &mut Bencher) {
    use linked_list::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push(i);
        }
        a
    });
}
#[bench]
fn bench_std_linked_list(b: &mut Bencher) {
    use std::collections::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push_front(i);
        }
        a
    });
}


#[bench]
fn bench_persistent_list_iter(b: &mut Bencher) {
    use persistent_list::PersistentList;

    let mut a = PersistentList::new();
    let mut index = SIZE;

    for i in 0..SIZE {
        a = a.push(i);
    }

    b.iter(move || {
        index = SIZE;
        for i in a.iter() {
            index -= 1;
            assert_eq!(i, &index);
        }
    });
}
#[bench]
fn bench_linked_list_iter(b: &mut Bencher) {
    use linked_list::LinkedList;

    let mut a = LinkedList::new();
    let mut index = SIZE;

    for i in 0..SIZE {
        a.push_front(i);
    }

    b.iter(move || {
        index = SIZE;
        for i in a.iter() {
            index -= 1;
            assert_eq!(i, &index);
        }
    });
}
#[bench]
fn bench_std_linked_list_iter(b: &mut Bencher) {
    use std::collections::LinkedList;

    let mut a = LinkedList::new();
    let mut index = SIZE;

    for i in 0..SIZE {
        a.push_front(i);
    }

    b.iter(move || {
        index = SIZE;
        for i in a.iter() {
            index -= 1;
            assert_eq!(i, &index);
        }
    });
}
