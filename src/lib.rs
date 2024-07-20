use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

/// A vector that can only be indexed by a specific type
///
/// Used for air-tight indexing with newtypes
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub struct TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    inner: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K, V> TypeKeyVec<K, V>
where
    K: Into<usize>,
{
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
    pub fn push(&mut self, value: V) {
        self.inner.push(value);
    }

    #[inline]
    pub fn get(&self, key: K) -> Option<&V> {
        self.inner.get(key.into())
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
    pub fn iter(&self) -> std::slice::Iter<V> {
        self.inner.iter()
    }
}

impl<K, V> Default for TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<K, V: Clone> Clone for TypeKeyVec<K, V>
where
    K: Into<usize>,
{
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

impl<K, V> From<Vec<V>> for TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    fn from(inner: Vec<V>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'data, K, V> IntoIterator for &'data TypeKeyVec<K, V>
where
    K: Into<usize>,
{
    type Item = &'data V;
    type IntoIter = <&'data Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
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
        self.inner.par_iter()
    }
}
