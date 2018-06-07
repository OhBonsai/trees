//! `Forest` composed of disjoint `Tree`s.

use super::{Node,Tree,Iter,IterMut,SubtreeIter};
use super::Walk;
use rust::*;

/// A nullable forest
pub struct Forest<T> {
    sub : *mut Node<T>,
    mark : super::heap::Phantom<T>,
}

impl<T> Forest<T> {
    /// Makes an empty `Forest`
    #[inline] pub fn new() -> Forest<T> { Self::from( null_mut() )}

    /// Returns `true` if the `Forest` is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    /// let mut forest = fr();
    /// assert!( forest.is_empty() );
    /// forest.push_back( tr(1) ); 
    /// assert!( !forest.is_empty() );
    /// ```
    #[inline] pub fn is_empty( &self ) -> bool { self.sub.is_null() }

    #[inline] pub(crate) fn set_child( &mut self, node: *mut Node<T> ) { self.sub = node; }
    #[inline] pub(crate) fn from( node: *mut Node<T> ) -> Self { Forest{ sub: node, mark: PhantomData } }
    #[inline] pub(crate) fn clear( &mut self ) { self.sub = null_mut(); }

    #[inline] pub(crate) unsafe fn set_sib( &mut self, sib: *mut Node<T> ) {
        (*self.tail()).sib = sib;
    }

    #[inline] pub(crate) unsafe fn head ( &self ) -> *mut Node<T> { (*self.sub).sib }
    #[inline] pub(crate) fn tail ( &self ) -> *mut Node<T> { self.sub }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Node<T> { (*self.head()).sib }

    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn adopt( &mut self, child: *mut Node<T> ) {
        unsafe {
            (*self.tail()).sib = child;
        }
    }

    /// Returns the last child.
    ///
    pub fn last( &self ) -> Option<&Node<T>> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some( &*self.tail() )}
        }
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// forest.push_front( tr(3) );
    /// assert_eq!( forest.to_string(), "( 3 1 2 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        if self.is_empty() {
            self.set_child( tree.root );
        } else { unsafe {
            tree.set_sib( self.head() );
            self.adopt( tree.root );
        }}
        tree.clear();
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// forest.push_back( tr(3) );
    /// assert_eq!( forest.to_string(), "( 1 2 3 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        if !self.is_empty() {
            unsafe {
                tree.set_sib( self.head() );
                self.adopt( tree.root );
            }
        }
        self.set_child( tree.root );
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// assert_eq!( forest.pop_front(), Some( tr(1) ));
    /// assert_eq!( forest.to_string(), "( 2 )" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_empty() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.clear();
            } else {
                (*self.tail()).sib = self.new_head();
            }
            (*front).reset_sib();
            Some( Tree::from( front ))
        }}
    }

    /// merge the forest at front
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest1 = -tr(0)-tr(1);
    /// let mut forest2 = -tr(2)-tr(3);
    /// forest1.prepend( forest2 );
    /// assert_eq!( forest1.to_string(), "( 2 3 0 1 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_empty() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
            }}
            forest.clear();
        }
    }

    /// merge the forest at back
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest1 = -tr(0)-tr(1);
    /// let mut forest2 = -tr(2)-tr(3);
    /// forest1.append( forest2 );
    /// assert_eq!( forest1.to_string(), "( 0 1 2 3 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if !self.is_empty() { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
            }}
            self.set_child( forest.tail() );
            forest.clear();
        }
    }

    /// Provides a forward iterator over `Forest`'s `Tree`s' root `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let forest = -tr(1)-tr(2);
    /// let mut iter = forest.children();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn children<'a>( &self ) -> Iter<'a,T> {
        if self.is_empty() {
            Iter::new( null(), null() )
        } else { unsafe {
            Iter::new( self.head(), self.tail() )
        }}
    }

    /// Provides a forward iterator over `Forest`'s `Tree`s' root `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// for child in forest.children_mut() { child.data *= 10; }
    /// assert_eq!( forest.to_string(), "( 10 20 )" );
    /// ```
    #[inline] pub fn children_mut<'a>( &mut self ) -> IterMut<'a,T> {
        if self.is_empty() {
            IterMut::new( null_mut(), null_mut() )
        } else { unsafe {
            IterMut::new( self.head(), self.tail() )
        }}
    }

    /// Provide an iterator over the `Forest`'s subtrees for insert/remove at any position.
    /// See `Subtree`'s document for more.
    #[inline] pub fn subtrees<'a>( &mut self ) -> SubtreeIter<'a,T> {
        unsafe {
            if self.is_empty() {
                SubtreeIter {
                    next: null_mut(), curr: null_mut(), prev: null_mut(), tail: null_mut(),
                    sub : &mut self.sub as *mut *mut Node<T>,
                    mark: PhantomData,
                }
            } else {
                SubtreeIter {
                    next : self.head(),
                    curr : null_mut(),
                    prev : self.sub,
                    tail : self.sub,
                    sub : &mut self.sub as *mut *mut Node<T>,
                    mark : PhantomData,
                }
            }
        }
    }

    /// Depth first search on `Forest`.
    /// Preorder or postorder at will.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut dfs = forest.walk();
    /// assert_eq!( dfs.next(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::Leaf ( tr(2).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::Leaf ( tr(3).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::End  ( (tr(1)/tr(2)/tr(3)).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::Leaf ( tr(5).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::Leaf ( tr(6).root() )));
    /// assert_eq!( dfs.next(), Some( Visit::End  ( (tr(4)/tr(5)/tr(6)).root() )));
    /// assert_eq!( dfs.next(), None );
    /// ```
    #[inline] pub fn walk( &self ) -> Walk<T> {
        if self.is_empty() {
            Walk::default()
        } else { unsafe {
            Walk::new( &*self.tail() )
        }}
    }
}

impl<T:Clone> Clone for Forest<T> {
    fn clone( &self ) -> Self {
        let mut forest = Forest::<T>::new();
        for child in self.children() {
            forest.push_back( child.to_owned() );
        }
        forest
    }
}

impl<T> Default for Forest<T> { #[inline] fn default() -> Self { Self::new() }}

impl<T> Drop for Forest<T> {
    fn drop( &mut self ) {
        while let Some(_) = self.pop_front() {}
    }
}

pub struct IntoIter<T> {
    forest : Forest<T>,
    marker : PhantomData<Tree<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = Tree<T>;

    #[inline] fn next( &mut self ) -> Option<Tree<T>> { self.forest.pop_front() }
}

impl<T> IntoIterator for Forest<T> {
    type Item = Tree<T>;
    type IntoIter = IntoIter<T>;

    #[inline] fn into_iter( self ) -> IntoIter<T> { IntoIter{ forest: self, marker: PhantomData }}
}

impl<T> FromIterator<Tree<T>> for Forest<T> {
   fn from_iter<I:IntoIterator<Item=Tree<T>>>( iter: I ) -> Self {
        let mut iter = iter.into_iter();
        let mut children = Forest::<T>::new();
        while let Some( node ) = iter.next() {
            children.push_back( node );
        }
        children
    }
}

impl<T> Extend<Tree<T>> for Forest<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T:Debug> Debug for Forest<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
    if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Forest<T> {
    fn eq( &self, other: &Self ) -> bool { self.children().eq( other.children() )}
    fn ne( &self, other: &Self ) -> bool { self.children().ne( other.children() )}
}

impl<T:Eq> Eq for Forest<T> {}

impl<T:PartialOrd> PartialOrd for Forest<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        self.children().partial_cmp( other.children() )
    }
}

impl<T:Ord> Ord for Forest<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        self.children().cmp( other.children() )
    }
}

impl<T:Hash> Hash for Forest<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        for child in self.children() {
            child.hash( state );
        }
    }
}

unsafe impl<T:Send> Send for Forest<T> {}
unsafe impl<T:Sync> Sync for Forest<T> {}