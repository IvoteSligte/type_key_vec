use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A vector that can only be indexed by a specific type
///
/// Used for air-tight indexing with newtypes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub fn push(&mut self, value: V) {
        self.inner.push(value);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.inner.get(key.into())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

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
