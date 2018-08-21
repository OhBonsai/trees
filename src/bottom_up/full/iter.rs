use super::Node;
use rust::*;

/// An iterator over the sub `Node`s of a `Node` or `Forest`.
///
/// This `struct` is created by [`Node::iter`] and [`Forest::iter`].
/// See its document for more.
///
/// [`Node::iter`]: struct.Node.html#method.iter
/// [`Forest::iter`]: struct.Forest.html#method.iter
pub struct Iter<'a, T:'a> {
    head : *const Node<T>,
    tail : *const Node<T>,
    len  : usize,
    mark : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    #[inline] fn next( &mut self ) -> Option<&'a Node<T>> {
        if self.head.is_null() {
             None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null()
            } else {
                (*node).next
            };
            self.len -= 1;
            Some( &*node )
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}

impl<'a, T:'a> Iter<'a, T> {
    #[inline] pub(crate) fn new( head: *const Node<T>, tail: *const Node<T>, len: usize ) -> Self {
        Iter{ head, tail, len, mark: PhantomData }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> FusedIterator for Iter<'a, T> {}

/// A mutable iterator over the sub `Node`s of a `Node` or `Forest`.
///
/// This `struct` is created by [`Node::iter_mut`] and [`Forest::iter_mut`].
///  See its document for more.
///
/// [`Node::iter`]: struct.Node.html#method.iter_mut
/// [`Forest::iter`]: struct.Forest.html#method.iter_mut
pub struct IterMut<'a, T:'a> {
    head : *mut Node<T>,
    tail : *mut Node<T>,
    len  : usize,
    mark : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    #[inline] fn next( &mut self ) -> Option<&'a mut Node<T>> {
        if self.head.is_null() {
             None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null_mut()
            } else {
                (*node).next
            };
            self.len -= 1;
            Some( &mut *node )
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}

impl<'a, T:'a> IterMut<'a, T> {
    #[inline] pub(crate) fn new( head: *mut Node<T>, tail: *mut Node<T>, len: usize ) -> Self {
        IterMut{ head, tail, len, mark: PhantomData }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

unsafe impl<'a, T:Sync> Send for Iter<'a, T> {}
unsafe impl<'a, T:Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T:Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T:Sync> Sync for IterMut<'a, T> {}
