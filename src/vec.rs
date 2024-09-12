use std::{
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

use crate::TypeKeySlice;

#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;

/// A vector that can only be indexed by a specific type
///
/// Used for air-tight indexing with newtypes
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub struct TypeKeyVec<K, V> {
    inner: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K, V> TypeKeyVec<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn push(&mut self, value: V) {
        self.inner.push(value);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<V> {
        self.inner.iter()
    }

    #[inline]
    pub fn into_vec(self) -> Vec<V> {
        self.inner
    }
}

impl<K, V> TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    #[inline]
    pub fn get(&self, key: K) -> Option<&V> {
        self.inner.get(key.into())
    }

    #[inline]
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.inner.get_mut(key.into())
    }
}

impl<K, V> Default for TypeKeyVec<K, V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<K, V: Clone> Clone for TypeKeyVec<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            phantom: PhantomData,
        }
    }
}

impl<K, V> Index<K> for TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.inner[index.into()]
    }
}

impl<K, V> IndexMut<K> for TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.inner[index.into()]
    }
}

impl<K, V> From<Vec<V>> for TypeKeyVec<K, V> {
    fn from(inner: Vec<V>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<K, V> Deref for TypeKeyVec<K, V> {
    type Target = TypeKeySlice<K, V>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_slice().as_ref()
    }
}

impl<K, V> IntoIterator for TypeKeyVec<K, V> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'data, K, V> IntoIterator for &'data TypeKeyVec<K, V> {
    type Item = &'data V;
    type IntoIter = <&'data Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'data, K, V> IntoIterator for &'data mut TypeKeyVec<K, V> {
    type Item = &'data mut V;
    type IntoIter = <&'data mut Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<K, V> IntoParallelIterator for TypeKeyVec<K, V>
where
    K: Into<usize>,
    V: Send,
{
    type Item = V;
    type Iter = <Vec<V> as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V> IntoParallelIterator for &'data TypeKeyVec<K, V>
where
    K: Into<usize>,
    V: Send + Sync,
{
    type Item = &'data V;
    type Iter = <&'data Vec<V> as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        (&self.inner).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V> IntoParallelIterator for &'data mut TypeKeyVec<K, V>
where
    K: Into<usize>,
    V: Send,
{
    type Item = &'data mut V;
    type Iter = <&'data mut Vec<V> as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        (&mut self.inner).into_par_iter()
    }
}
