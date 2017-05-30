use alloc::arc::Arc;

use core::marker::PhantomData;

use collection_traits::*;


struct Node<T> {
    value: T,
    next: Option<Arc<Node<T>>>,
}

impl<T> Node<T> {

    #[inline(always)]
    fn new(value: T, next: Option<Arc<Node<T>>>) -> Self {
        Node {
            value: value,
            next: next,
        }
    }
}


pub struct PersistentList<T> {
    root: Option<Arc<Node<T>>>,
    len: usize,
}

impl<T> PersistentList<T> {

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

    #[inline]
    pub fn pop_unwrap(self) -> (Self, Option<T>) {
        let len = self.len;

        match self.root {
            Some(root) => {
                let new_root = root.next.clone();

                match Arc::try_unwrap(root) {
                    Ok(node) => (
                        PersistentList {
                            root: new_root,
                            len: len - 1usize,
                        },
                        Some(node.value)
                    ),
                    Err(root) => (
                        PersistentList {
                            root: Some(root),
                            len: len,
                        },
                        None
                    ),
                }
            },
            None => (self, None),
        }
    }
}

impl<T> Clone for PersistentList<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        PersistentList {
            root: self.root.clone(),
            len: self.len,
        }
    }
}

impl<T> Collection for PersistentList<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Stack<T> for PersistentList<T> {

    #[inline]
    fn push(&self, value: T) -> Self {
        match self.root {
            Some(ref root) => {
                PersistentList {
                    root: Some(Arc::new(Node::new(value, Some(root.clone())))),
                    len: self.len + 1usize,
                }
            },
            None => {
                PersistentList {
                    root: Some(Arc::new(Node::new(value, None))),
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

impl<'a, T: 'a> Iterable<'a, &'a T> for PersistentList<T> {
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
pub struct Iter<'a, T: 'a> {
    root: &'a Option<Arc<Node<T>>>,
    len: usize,
    phantom_data: PhantomData<&'a Arc<Node<T>>>,
}

unsafe impl<'a, T: 'a + Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: 'a + Sync> Sync for Iter<'a, T> {}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
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
