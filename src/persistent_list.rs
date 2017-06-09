use alloc::arc::Arc;
use core::marker::PhantomData;

use collection_traits::*;


struct Node<T: 'static> {
    value: T,
    next: Option<Arc<Node<T>>>,
}

impl<T: 'static> Node<T> {

    #[inline(always)]
    fn new(value: T, next: Option<Arc<Node<T>>>) -> Arc<Self> {
        Arc::new(Node {
            value: value,
            next: next,
        })
    }
}


pub struct PersistentList<T: 'static> {
    root: Option<Arc<Node<T>>>,
    len: usize,
}

impl<T: 'static> PersistentList<T> {

    #[inline(always)]
    pub fn new() -> Self {
        PersistentList {
            root: None,
            len: 0usize,
        }
    }

    #[inline]
    pub fn pop_and_top(&self) -> (Self, Option<&T>) {
        match self.root {
            Some(ref root) => (
                PersistentList {
                    root: root.next.clone(),
                    len: self.len - 1,
                },
                Some(&root.value)
            ),
            None => (PersistentList::new(), None),
        }
    }
}

impl<T: 'static> Clone for PersistentList<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        PersistentList {
            root: self.root.clone(),
            len: self.len,
        }
    }
}

impl<T: 'static> Collection for PersistentList<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }
}

impl<T: 'static> Stack<T> for PersistentList<T> {

    #[inline]
    fn push(&self, value: T) -> Self {
        match self.root {
            Some(ref root) => {
                PersistentList {
                    root: Some(Node::new(value, Some(root.clone()))),
                    len: self.len + 1usize,
                }
            },
            None => {
                PersistentList {
                    root: Some(Node::new(value, None)),
                    len: self.len + 1usize,
                }
            },
        }
    }

    #[inline]
    fn pop(&self) -> Self {
        match self.root {
            Some(ref root) => {
                PersistentList {
                    root: root.next.clone(),
                    len: self.len - 1,
                }
            },
            None => PersistentList::new(),
        }
    }

    #[inline]
    fn top(&self) -> Option<&T> {
        match self.root {
            Some(ref root) => Some(&root.value),
            None => None,
        }
    }
}

impl<'a, T: 'static> Iterable<'a, &'a T> for PersistentList<T> {
    type Iter = Iter<'a, T>;

    fn iter(&'a self) -> Self::Iter {
        Iter {
            root: &self.root,
            len: self.len,
            phantom_data: PhantomData,
        }
    }
}


#[derive(Clone)]
pub struct Iter<'a, T: 'static> {
    root: &'a Option<Arc<Node<T>>>,
    len: usize,
    phantom_data: PhantomData<&'a Arc<Node<T>>>,
}

unsafe impl<'a, T: 'a + Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: 'a+ Sync> Sync for Iter<'a, T> {}

impl<'a, T: 'static> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.root.as_ref().map(|root| {
                self.root = &root.next;
                self.len -= 1;
                &root.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
