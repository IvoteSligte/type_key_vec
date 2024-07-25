use std::{marker::PhantomData, ops::Index};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

/// A vector that can only be indexed by a specific type
///
/// Used for air-tight indexing with newtypes
#[derive(Debug)]
pub struct TypeKeySlice<K, V>
where
    K: Into<usize>,
{
    phantom: PhantomData<K>,
    inner: [V],
}

impl<K, V> TypeKeySlice<K, V>
where
    K: Into<usize>,
{
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

impl<K, V> Index<K> for TypeKeySlice<K, V>
where
    K: Into<usize>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.inner[index.into()]
    }
}

impl<K, V> AsRef<[V]> for TypeKeySlice<K, V>
where
    K: Into<usize>,
{
    fn as_ref(&self) -> &[V] {
        &self.inner
    }
}

impl<K, V> AsRef<TypeKeySlice<K, V>> for [V]
where
    K: Into<usize>,
{
    fn as_ref(&self) -> &TypeKeySlice<K, V> {
        // SAFETY: same size, alignment, etc
        unsafe { std::mem::transmute(self) }
    }
}

impl<'data, K, V> IntoIterator for &'data TypeKeySlice<K, V>
where
    K: Into<usize>,
{
    type Item = &'data V;
    type IntoIter = <&'data [V] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V> IntoParallelIterator for &'data TypeKeySlice<K, V>
where
    K: Into<usize>,
    V: Send + Sync,
{
    type Item = &'data V;
    type Iter = <&'data [V] as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.par_iter()
    }
}
