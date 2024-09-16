use std::{marker::PhantomData, ops::Index};

#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;

/// A vector that can only be indexed by a specific type
///
/// Used for air-tight indexing with newtypes
#[derive(Debug)]
pub struct TypeKeySlice<K, V> {
    phantom: PhantomData<K>,
    inner: [V],
}

impl<K, V> TypeKeySlice<K, V> {
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn fill(&mut self, value: V)
    where
        V: Clone,
    {
        self.inner.fill(value);
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<V> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<V> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn enumerate(&self) -> Enumerate<K, V> {
        Enumerate {
            iter: self.iter().enumerate(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[V] {
        &self.inner
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [V] {
        &mut self.inner
    }
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
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.inner.get_mut(key.into())
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

impl<K, V> AsRef<TypeKeySlice<K, V>> for [V] {
    fn as_ref(&self) -> &TypeKeySlice<K, V> {
        // SAFETY: same size, alignment, etc
        unsafe { std::mem::transmute(self) }
    }
}

impl<K, V> AsMut<TypeKeySlice<K, V>> for [V] {
    fn as_mut(&mut self) -> &mut TypeKeySlice<K, V> {
        // SAFETY: same size, alignment, etc
        unsafe { std::mem::transmute(self) }
    }
}

impl<'data, K, V> IntoIterator for &'data TypeKeySlice<K, V> {
    type Item = &'data V;
    type IntoIter = <&'data [V] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'data, K, V> IntoIterator for &'data mut TypeKeySlice<K, V> {
    type Item = &'data mut V;
    type IntoIter = <&'data mut [V] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V> IntoParallelIterator for &'data TypeKeySlice<K, V>
where
    V: Send + Sync,
{
    type Item = &'data V;
    type Iter = <&'data [V] as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        (&self.inner).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V> IntoParallelIterator for &'data mut TypeKeySlice<K, V>
where
    V: Send + Sync,
{
    type Item = &'data mut V;
    type Iter = <&'data mut [V] as IntoParallelIterator>::Iter;

    fn into_par_iter(self) -> Self::Iter {
        (&mut self.inner).into_par_iter()
    }
}

pub struct Enumerate<'a, K, V> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, V>>,
    phantom: PhantomData<K>,
}

impl<'a, K, V> Iterator for Enumerate<'a, K, V>
where
    K: From<usize>,
{
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, v)| (K::from(i), v))
    }
}
