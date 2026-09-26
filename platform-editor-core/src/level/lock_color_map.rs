use std::iter::Enumerate;

use crate::level::LockColor;

/// A map whose keys are lock colors and all have entries.
///
/// In this map, values are stored in the order of keys in [`LockColor::ALL`]
/// (i.e. the order of colors in the [`LockColor`] enum).
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FilledLockColorMap<V>(pub [V; LockColor::ALL.len()]);

impl<V> FilledLockColorMap<V> {
    pub fn get(&self, color: LockColor) -> &V {
        &self.0[color as usize]
    }

    pub fn get_mut(&mut self, color: LockColor) -> &mut V {
        &mut self.0[color as usize]
    }

    pub fn set(&mut self, color: LockColor, value: V) {
        self.0[color as usize] = value
    }
}

/// A map whose keys are lock colors. Not all keys have to be filled
/// in this map.
#[derive(Debug, PartialEq, Clone)]
pub struct LockColorMap<V>(FilledLockColorMap<Option<V>>);

impl<V> LockColorMap<V> {
    /// Returns an empty `LockColorMap`.
    pub fn new() -> Self {
        Self(FilledLockColorMap([const { None }; LockColor::ALL.len()]))
    }

    /// Clears this map.
    pub fn clear(&mut self) {
        self.0 = FilledLockColorMap([const { None }; LockColor::ALL.len()]);
    }

    /// Returns a reference to the value corresponding to the color.
    pub fn get(&self, color: LockColor) -> Option<&V> {
        self.0.get(color).as_ref()
    }

    /// Returns a mutable reference to the value corresponding to the color.
    pub fn get_mut(&mut self, color: LockColor) -> Option<&mut V> {
        self.0.get_mut(color).as_mut()
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, color: LockColor, value: V) {
        self.0.set(color, Some(value));
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, color: LockColor) -> bool {
        self.get(color).is_some()
    }
}

impl<V> Default for LockColorMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ITERATOR IMPLEMENTATIONS

type InnerEnumerateIter<'a, T> = Enumerate<std::slice::Iter<'a, T>>;
type InnerEnumerateIterMut<'a, T> = Enumerate<std::slice::IterMut<'a, T>>;

fn transmute(i: usize) -> LockColor {
    // SAFETY: The indices provided by `enumerate` (guaranteed by all calls of
    // this function being from them) will always be less than
    // the length of the array slice in the map.

    unsafe { std::mem::transmute(i as u8) }
}

/// An [`Iterator`] implementation for filled lock color maps.
pub struct FilledIter<'a, V> {
    inner: InnerEnumerateIter<'a, V>,
}

/// An [`Iterator`] implementation with mutable values for filled lock color maps.
pub struct FilledIterMut<'a, V> {
    inner: InnerEnumerateIterMut<'a, V>,
}

impl<'a, V> Iterator for FilledIter<'a, V> {
    type Item = (LockColor, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(i, v)| (transmute(i), v))
    }
}

impl<'a, V> Iterator for FilledIterMut<'a, V> {
    type Item = (LockColor, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(
            // SAFETY: The indices provided by `enumerate` will always be less than
            // the length of the array slice in the map.
            |(i, v)| (transmute(i), v),
        )
    }
}

impl<'a, V> IntoIterator for &'a FilledLockColorMap<V> {
    type Item = (LockColor, &'a V);

    type IntoIter = FilledIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        FilledIter {
            inner: self.0.iter().enumerate(),
        }
    }
}

impl<'a, V> IntoIterator for &'a mut FilledLockColorMap<V> {
    type Item = (LockColor, &'a mut V);

    type IntoIter = FilledIterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        FilledIterMut {
            inner: self.0.iter_mut().enumerate(),
        }
    }
}

fn filter_map_function<V>(tuple: (usize, &Option<V>)) -> Option<(usize, &V)> {
    tuple.1.as_ref().map(|v| (tuple.0, v))
}

fn filter_map_function_mut<V>(tuple: (usize, &mut Option<V>)) -> Option<(usize, &mut V)> {
    tuple.1.as_mut().map(|v| (tuple.0, v))
}

type FilterMapIterFn<'a, V> = fn((usize, &'a Option<V>)) -> Option<(usize, &'a V)>;
type FilterMapIterMutFn<'a, V> = fn((usize, &'a mut Option<V>)) -> Option<(usize, &'a mut V)>;

/// An [`Iterator`] implementation for flock color maps.
pub struct Iter<'a, V> {
    inner: std::iter::FilterMap<InnerEnumerateIter<'a, Option<V>>, FilterMapIterFn<'a, V>>,
}

/// An [`Iterator`] implementation with mutable values for lock color maps.
pub struct IterMut<'a, V> {
    inner: std::iter::FilterMap<InnerEnumerateIterMut<'a, Option<V>>, FilterMapIterMutFn<'a, V>>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (LockColor, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(
            // SAFETY: The indices provided by `enumerate` will always be less than
            // the length of the array slice in the map.
            |(i, v)| (transmute(i), v),
        )
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (LockColor, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(
            // SAFETY: The indices provided by `enumerate` will always be less than
            // the length of the array slice in the map.
            |(i, v)| (transmute(i), v),
        )
    }
}

impl<'a, V> IntoIterator for &'a LockColorMap<V> {
    type Item = (LockColor, &'a V);

    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.0.0.iter().enumerate().filter_map(filter_map_function),
        }
    }
}

impl<'a, V> IntoIterator for &'a mut LockColorMap<V> {
    type Item = (LockColor, &'a mut V);

    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self
                .0
                .0
                .iter_mut()
                .enumerate()
                .filter_map(filter_map_function_mut),
        }
    }
}
