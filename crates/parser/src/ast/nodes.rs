use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;
use std::collections::VecDeque;

use crate::{Item, Node};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Nodes<'a> {
    data: VecDeque<Node<'a>>,
}
impl<'a> Nodes<'a> {
    pub fn new() -> Nodes<'a> {
        Nodes::default()
    }

    pub fn contains(&mut self, node: &Node<'a>) -> bool {
        let node = node.clone();
        self.data.contains(node)
    }

    pub fn add(&mut self, node: Node<'a>) {
        let node = node.clone();
        if !self.data.contains(&node) {
            self.data.push_back(node.clone())
        }
    }

    pub fn first(&self) -> Option<&Node<'a>> {
        self.data.front()
    }

    pub fn last(&self) -> Option<&Node<'a>> {
        self.data.back()
    }

    pub fn push(&mut self, node: Node<'a>) {
        self.add(node)
    }

    pub fn push_back(&mut self, node: Node<'a>) {
        self.data.push_back(node)
    }

    pub fn push_front(&mut self, node: Node<'a>) {
        self.data.push_front(node)
    }

    pub fn insert(&mut self, node: Node<'a>) {
        self.push(node)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter_mut(&mut self) -> NodesIterMut {
        NodesIterMut::new(&mut self.data)
    }

    pub fn iter(&self) -> NodesIter {
        NodesIter::new(&self.data)
    }
}

pub struct NodesIterMut<'a> {
    ptr: NonNull<Node<'a>>,
    end_or_len: *mut Node<'a>,
    _marker: PhantomData<&'a Node<'a>>,
}
impl<'a> NodesIterMut<'a> {
    pub fn new(slice: &mut [Node<'a>]) -> Self {
        let len = slice.len();
        let ptr: NonNull<Node<'a>> = NonNull::from_ref(slice).cast();
        unsafe {
            let end_or_len = ptr.as_ptr().add(len);

            Self {
                ptr,
                end_or_len,
                _marker: PhantomData,
            }
        }
    }
}
impl<'a> Iterator for NodesIterMut<'a> {
    type Item = &'a mut Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.addr().get() == self.end_or_len.addr() {
            None
        } else {
            unsafe {
                let item = self.ptr.as_mut();
                let ptr = self.ptr.add(1);
                self.ptr = ptr;
                Some(item)
            }
        }
    }
}
pub struct NodesIter<'a> {
    ptr: NonNull<Node<'a>>,
    end_or_len: *const Node<'a>,
    _marker: PhantomData<&'a Node<'a>>,
}
impl<'a> NodesIter<'a> {
    pub fn new(slice: &'a [Node<'a>]) -> Self {
        let len = slice.len();
        let ptr: NonNull<Node<'a>> = NonNull::from_ref(slice).cast();
        unsafe {
            let end_or_len = ptr.as_ptr().add(len);
            Self {
                ptr,
                end_or_len,
                _marker: PhantomData,
            }
        }
    }
}
impl<'a> Iterator for NodesIter<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.addr().get() == self.end_or_len.addr() {
            None
        } else {
            unsafe {
                let item = self.ptr.as_ref();
                let ptr = self.ptr.add(1);
                self.ptr = ptr;
                Some(item)
            }
        }
    }
}

impl<'a, 'b> Extend<&'a &'b Node<'a>> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = &'a &'b Node<'a>>>(&mut self, iter: T) {
        for node in iter {
            let node = node.clone();
            if !self.contains(&node) {
                self.push(node);
            }
        }
    }
}
impl<'a, 'b> Extend<&'a &'b str> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = &'a &'b str>>(&mut self, iter: T) {
        for node in iter.into_iter() {
            let node = node.clone();
            if !self.contains(&node) {
                self.push(node);
            }
        }
    }
}
impl<'a> Extend<&'a Node<'a>> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = &'a Node<'a>>>(&mut self, iter: T) {
        for node in iter.into_iter() {
            let node = node.clone();
            if !self.contains(node) {
                self.push(node);
            }
        }
    }
}

impl<'a> Extend<&'a Node<'a>> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = &'a Node<'a>>>(&mut self, iter: T) {
        for node in iter.into_iter() {
            if !self.contains(node) {
                self.push(node.clone());
            }
        }
    }
}
impl<'a> Extend<Node<'a>> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = Node<'a>>>(&mut self, iter: T) {
        for node in iter.into_iter() {
            if !self.contains(node) {
                self.push(node);
            }
        }
    }
}
impl<'a> Extend<&'a Nodes<'a>> for Nodes<'a> {
    fn extend<T: IntoIterator<Item = &'a Nodes<'a>>>(&mut self, iter: T) {
        for set in iter.into_iter() {
            self.extend(set.iter());
        }
    }
}
impl<'a> IntoIterator for Nodes<'a> {
    type IntoIter = std::vec::IntoIter<Node<'a>>;
    type Item = Node<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.clone().into_iter()
    }
}

impl<'a> IntoIterator for &'a Nodes<'a> {
    type IntoIter = NodesIter<'a>;
    type Item = &'a Node<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> From<VecDeque<Node<'a>>> for Nodes<'a> {
    fn from(iter: VecDeque<Node<'a>>) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, const N: usize> From<[&'a Node<'a>; N]> for Nodes<'a> {
    fn from(iter: [&'a Node<'a>; N]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b> From<&'a [&'b str]> for Nodes<'a> {
    fn from(iter: &'a [&'b str]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b, const N: usize> From<&'a [&'b str; N]> for Nodes<'a> {
    fn from(iter: &'a [&'b str; N]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<const N: usize> From<[Node<'a>; N]> for Nodes<'a> {
    fn from(iter: [Node<'a>; N]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, const N: usize> From<[&'a Node<'a>; N]> for Nodes<'a> {
    fn from(iter: [&'a Node<'a>; N]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}

impl<'a, 'b> From<&'a [&'b Node<'a>]> for Nodes<'a> {
    fn from(iter: &'a [&'b Node<'a>]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b, const N: usize> From<&'a [&'b Node<'a>; N]> for Nodes<'a> {
    fn from(iter: &'a [&'b Node<'a>; N]) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<Node<'a>> for Nodes<'a> {
    fn from_iter<I: IntoIterator<Item = Node<'a>>>(iter: I) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a Nodes<'a>> for Nodes<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Nodes<'a>>>(iter: I) -> Nodes<'a> {
        let mut buf = Nodes::new();
        for set in iter {
            buf.extend(set.iter());
        }
        buf
    }
}
impl<'a> FromIterator<Nodes<'a>> for Nodes<'a> {
    fn from_iter<I: IntoIterator<Item = Nodes<'a>>>(iter: I) -> Nodes<'a> {
        let mut buf = Nodes::new();
        for set in iter {
            buf.extend(set.iter());
        }
        buf
    }
}
impl<'a> FromIterator<&'a Node<'a>> for Nodes<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Node<'a>>>(iter: I) -> Nodes<'a> {
        let mut buf = Nodes::new();
        buf.extend(iter.into_iter().map(Clone::clone).collect::<VecDeque<Node<'a>>>());
        buf
    }
}

impl<'a> Index<usize> for Nodes<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.data, index)
    }
}

impl<'a> IndexMut<usize> for Nodes<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.data, index)
    }
}

impl<'a> Deref for Nodes<'a> {
    type Target = [Node<'a>];

    fn deref(&self) -> &[Node<'a>] {
        self.data.as_slices()
    }
}

impl<'a> DerefMut for Nodes<'a> {
    fn deref_mut(&mut self) -> &mut [Node<'a>] {
        self.data.as_mut_slices()
    }
}

impl<'a> std::fmt::Debug for Nodes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", &self.data)
    }
}

impl<'a, const N: usize> PartialEq<[&'a Node<'a>; N]> for Nodes<'a> {
    fn eq(&self, other: &[&'a Node<'a>; N]) -> bool {
        let count = self.len();
        if count != N {
            return false;
        }
        let mut index = 0;
        while index < N {
            if self[index].clone() != other[index].clone() {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl<'a> PartialEq<VecDeque<Item<'a>>> for Nodes<'a> {
    fn eq(&self, other: &'a VecDeque<Item<'a>>) -> bool {
        let count = self.len();
        if count != other.len() {
            return false;
        }
        let mut index = 0;
        while index < count {
            if self[index].clone() != other[index].clone() {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl<'a> PartialEq<VecDeque<Node<'a>>> for Nodes<'a> {
    fn eq(&self, other: &'a VecDeque<Node<'a>>) -> bool {
        let count = self.len();
        if count != other.len() {
            return false;
        }
        let mut index = 0;
        while index < count {
            if self[index].clone() != other[index].clone() {
                return false;
            }
            index += 1;
        }
        true
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! oss {
    ($( $arg:expr ),* ) => {{
        let mut set = $crate::<Nodes::<'a>>::new();
        $(
            set.push($arg);
        )*
        set
    }};
}
