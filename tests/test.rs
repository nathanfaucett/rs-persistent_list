extern crate collection_traits;

extern crate persistent_list;


use collection_traits::*;

use persistent_list::PersistentList;


#[test]
fn test_push_pop() {
    let a = PersistentList::new();
    let b = a.push(1);
    let c = b.push(2);
    let d = c.pop();

    assert_eq!(a.len(), 0);

    assert_eq!(*b.top().unwrap(), 1);
    assert_eq!(b.len(), 1);

    assert_eq!(*c.top().unwrap(), 2);
    assert_eq!(c.len(), 2);

    assert_eq!(*d.top().unwrap(), 1);
    assert_eq!(d.len(), 1);
}

#[test]
fn test_iter() {
    let a = PersistentList::new();
    let b = a.push(3);
    let c = b.push(2);
    let d = c.push(1);
    let e = d.push(0);
    let mut i = 0;

    for value in e.iter() {
        assert_eq!(*value, i);
        i += 1;
    }
}
