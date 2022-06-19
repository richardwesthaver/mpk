//! mpk_vm/m/ -- slab.rs
//! A port of slab with custom allocator support
//! https://github.com/tokio-rs/slab
use std::alloc::Allocator;
use std::vec::{self, Vec};
use core::iter::{self, FusedIterator};
use core::marker::PhantomData;
// use core::iter::FromIterator;
use core::{fmt, mem, ops, slice};

use bumpalo::Bump;

/// Pre-allocated storage for a uniform data type
#[derive(Clone)]
pub struct Slab<'arena, T: 'arena, A: Allocator = &'arena Bump> {
  // Chunk of memory
  entries: Vec<Entry<T>, A>,

  // Number of Filled elements currently in the slab
  len: usize,

  // Offset of the next available slot in the slab. Set to the slab's
  // capacity when the slab is full.
  next: usize,
  _marker: PhantomData<&'arena T>,
}

#[derive(Debug)]
pub struct VacantEntry<'entry, 'arena, T, A: Allocator> {
  slab: &'entry mut Slab<'arena, T, A>,
  key: usize,
}

/// A consuming iterator over the values stored in a `Slab`
pub struct IntoIter<T, A: Allocator> {
  entries: iter::Enumerate<vec::IntoIter<Entry<T>, A>>,
  len: usize,
}

/// An iterator over the values stored in the `Slab`
pub struct Iter<'a, T> {
  entries: iter::Enumerate<slice::Iter<'a, Entry<T>>>,
  len: usize,
}

impl<'a, T> Clone for Iter<'a, T> {
  fn clone(&self) -> Self {
    Self {
      entries: self.entries.clone(),
      len: self.len,
    }
  }
}

/// A mutable iterator over the values stored in the `Slab`
pub struct IterMut<'a, T> {
  entries: iter::Enumerate<slice::IterMut<'a, Entry<T>>>,
  len: usize,
}

/// A draining iterator for `Slab`
pub struct Drain<'a, T, A: Allocator> {
  inner: vec::Drain<'a, Entry<T>, A>,
  len: usize,
}

#[derive(Clone)]
enum Entry<T> {
  Vacant(usize),
  Occupied(T),
}

impl<'arena, T, A: Allocator> Slab<'arena, T, A> {
  pub fn new_in(alloc: &'arena Bump) -> Slab<T, &'arena Bump> {
    let entries = Vec::new_in(alloc);
    Slab {
      entries,
      next: 0,
      len: 0,
      _marker: PhantomData::<&'arena T>,
    }
  }

  pub fn with_capacity_in(capacity: usize, alloc: A) -> Slab<'arena, T, A> {
    let entries = Vec::with_capacity_in(capacity, alloc);
    Slab {
      entries,
      next: 0,
      len: 0,
      _marker: PhantomData::<&'arena T>,
    }
  }

  pub fn capacity(&self) -> usize {
    self.entries.capacity()
  }

  pub fn reserve(&mut self, additional: usize) {
    if self.capacity() - self.len >= additional {
      return;
    }
    let need_add = additional - (self.entries.len() - self.len);
    self.entries.reserve(need_add);
  }

  pub fn reserve_exact(&mut self, additional: usize) {
    if self.capacity() - self.len >= additional {
      return;
    }
    let need_add = additional - (self.entries.len() - self.len);
    self.entries.reserve_exact(need_add);
  }

  pub fn shrink_to_fit(&mut self) {
    // Remove all vacant entries after the last occupied one, so that
    // the capacity can be reduced to what is actually needed.
    // If the slab is empty the vector can simply be cleared, but that
    // optimization would not affect time complexity when T: Drop.
    let len_before = self.entries.len();
    while let Some(&Entry::Vacant(_)) = self.entries.last() {
      self.entries.pop();
    }

    // Removing entries breaks the list of vacant entries,
    // so it must be repaired
    if self.entries.len() != len_before {
      // Some vacant entries were removed, so the list now likely¹
      // either contains references to the removed entries, or has an
      // invalid end marker. Fix this by recreating the list.
      self.recreate_vacant_list();
      // ¹: If the removed entries formed the tail of the list, with the
      // most recently popped entry being the head of them, (so that its
      // index is now the end marker) the list is still valid.
      // Checking for that unlikely scenario of this infrequently called
      // is not worth the code complexity.
    }

    self.entries.shrink_to_fit();
  }

  fn recreate_vacant_list(&mut self) {
    self.next = self.entries.len();
    // We can stop once we've found all vacant entries
    let mut remaining_vacant = self.entries.len() - self.len;
    // Iterate in reverse order so that lower keys are at the start of
    // the vacant list. This way future shrinks are more likely to be
    // able to remove vacant entries.
    for (i, entry) in self.entries.iter_mut().enumerate().rev() {
      if remaining_vacant == 0 {
        break;
      }
      if let Entry::Vacant(ref mut next) = *entry {
        *next = self.next;
        self.next = i;
        remaining_vacant -= 1;
      }
    }
  }

  pub fn compact<F>(&mut self, mut rekey: F)
  where
    F: FnMut(&mut T, usize, usize) -> bool,
  {
    // If the closure unwinds, we need to restore a valid list of vacant entries
    struct CleanupGuard<'guard, 'arena, T, A: Allocator> {
      slab: &'guard mut Slab<'arena, T, A>,
      decrement: bool,
    }
    impl<'guard, T, A: Allocator> Drop for CleanupGuard<'guard, '_, T, A> {
      fn drop(&mut self) {
        if self.decrement {
          // Value was popped and not pushed back on
          self.slab.len -= 1;
        }
        self.slab.recreate_vacant_list();
      }
    }
    let mut guard = CleanupGuard {
      slab: self,
      decrement: true,
    };

    let mut occupied_until = 0;
    // While there are vacant entries
    while guard.slab.entries.len() > guard.slab.len {
      // Find a value that needs to be moved,
      // by popping entries until we find an occupied one.
      // (entries cannot be empty because 0 is not greater than anything)
      if let Some(Entry::Occupied(mut value)) = guard.slab.entries.pop() {
        // Found one, now find a vacant entry to move it to
        while let Some(&Entry::Occupied(_)) = guard.slab.entries.get(occupied_until) {
          occupied_until += 1;
        }
        // Let the caller try to update references to the key
        if !rekey(&mut value, guard.slab.entries.len(), occupied_until) {
          // Changing the key failed, so push the entry back on at its old index.
          guard.slab.entries.push(Entry::Occupied(value));
          guard.decrement = false;
          guard.slab.entries.shrink_to_fit();
          return;
          // Guard drop handles cleanup
        }
        // Put the value in its new spot
        guard.slab.entries[occupied_until] = Entry::Occupied(value);
        // ... and mark it as occupied (this is optional)
        occupied_until += 1;
      }
    }
    guard.slab.next = guard.slab.len;
    guard.slab.entries.shrink_to_fit();
    // Normal cleanup is not necessary
    mem::forget(guard);
  }

  pub fn clear(&mut self) {
    self.entries.clear();
    self.len = 0;
    self.next = 0;
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      entries: self.entries.iter().enumerate(),
      len: self.len,
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut {
      entries: self.entries.iter_mut().enumerate(),
      len: self.len,
    }
  }

  pub fn get(&self, key: usize) -> Option<&T> {
    match self.entries.get(key) {
      Some(&Entry::Occupied(ref val)) => Some(val),
      _ => None,
    }
  }

  pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
    match self.entries.get_mut(key) {
      Some(&mut Entry::Occupied(ref mut val)) => Some(val),
      _ => None,
    }
  }

  pub fn get2_mut(&mut self, key1: usize, key2: usize) -> Option<(&mut T, &mut T)> {
    assert!(key1 != key2);

    let (entry1, entry2);

    if key1 > key2 {
      let (slice1, slice2) = self.entries.split_at_mut(key1);
      entry1 = slice2.get_mut(0);
      entry2 = slice1.get_mut(key2);
    } else {
      let (slice1, slice2) = self.entries.split_at_mut(key2);
      entry1 = slice1.get_mut(key1);
      entry2 = slice2.get_mut(0);
    }

    match (entry1, entry2) {
      (
        Some(&mut Entry::Occupied(ref mut val1)),
        Some(&mut Entry::Occupied(ref mut val2)),
      ) => Some((val1, val2)),
      _ => None,
    }
  }

  pub unsafe fn get_unchecked(&self, key: usize) -> &T {
    match *self.entries.get_unchecked(key) {
      Entry::Occupied(ref val) => val,
      _ => unreachable!(),
    }
  }

  pub unsafe fn get_unchecked_mut(&mut self, key: usize) -> &mut T {
    match *self.entries.get_unchecked_mut(key) {
      Entry::Occupied(ref mut val) => val,
      _ => unreachable!(),
    }
  }

  pub unsafe fn get2_unchecked_mut(
    &mut self,
    key1: usize,
    key2: usize,
  ) -> (&mut T, &mut T) {
    debug_assert_ne!(key1, key2);
    let ptr = self.entries.as_mut_ptr();
    let ptr1 = ptr.add(key1);
    let ptr2 = ptr.add(key2);
    match (&mut *ptr1, &mut *ptr2) {
      (&mut Entry::Occupied(ref mut val1), &mut Entry::Occupied(ref mut val2)) => {
        (val1, val2)
      }
      _ => unreachable!(),
    }
  }

  pub fn key_of(&self, present_element: &T) -> usize {
    let element_ptr = present_element as *const T as usize;
    let base_ptr = self.entries.as_ptr() as usize;
    // Use wrapping subtraction in case the reference is bad
    let byte_offset = element_ptr.wrapping_sub(base_ptr);
    // The division rounds away any offset of T inside Entry
    // The size of Entry<T> is never zero even if T is due to Vacant(usize)
    let key = byte_offset / mem::size_of::<Entry<T>>();
    // Prevent returning unspecified (but out of bounds) values
    if key >= self.entries.len() {
      panic!("The reference points to a value outside this slab");
    }
    // The reference cannot point to a vacant entry, because then it would not be valid
    key
  }

  pub fn insert(&mut self, val: T) -> usize {
    let key = self.next;

    self.insert_at(key, val);

    key
  }

  pub fn vacant_key(&self) -> usize {
    self.next
  }

  pub fn vacant_entry(&'arena mut self) -> VacantEntry<'_, '_, T, A> {
    VacantEntry {
      key: self.next,
      slab: self,
    }
  }

  fn insert_at(&mut self, key: usize, val: T) {
    self.len += 1;

    if key == self.entries.len() {
      self.entries.push(Entry::Occupied(val));
      self.next = key + 1;
    } else {
      self.next = match self.entries.get(key) {
        Some(&Entry::Vacant(next)) => next,
        _ => unreachable!(),
      };
      self.entries[key] = Entry::Occupied(val);
    }
  }

  pub fn try_remove(&mut self, key: usize) -> Option<T> {
    if let Some(entry) = self.entries.get_mut(key) {
      // Swap the entry at the provided value
      let prev = mem::replace(entry, Entry::Vacant(self.next));

      match prev {
        Entry::Occupied(val) => {
          self.len -= 1;
          self.next = key;
          return val.into();
        }
        _ => {
          // Woops, the entry is actually vacant, restore the state
          *entry = prev;
        }
      }
    }
    None
  }

  pub fn remove(&mut self, key: usize) -> T {
    self.try_remove(key).expect("invalid key")
  }

  pub fn contains(&self, key: usize) -> bool {
    match self.entries.get(key) {
      Some(&Entry::Occupied(_)) => true,
      _ => false,
    }
  }

  pub fn retain<F>(&mut self, mut f: F)
  where
    F: FnMut(usize, &mut T) -> bool,
  {
    for i in 0..self.entries.len() {
      let keep = match self.entries[i] {
        Entry::Occupied(ref mut v) => f(i, v),
        _ => true,
      };

      if !keep {
        self.remove(i);
      }
    }
  }

  pub fn drain(&mut self) -> Drain<'_, T, A> {
    let old_len = self.len;
    self.len = 0;
    self.next = 0;
    Drain {
      inner: self.entries.drain(..),
      len: old_len,
    }
  }
}

impl<'arena, T, A: Allocator> ops::Index<usize> for Slab<'arena, T, A> {
  type Output = T;

  fn index(&self, key: usize) -> &T {
    match self.entries.get(key) {
      Some(&Entry::Occupied(ref v)) => v,
      _ => panic!("invalid key"),
    }
  }
}

impl<'arena, T, A: Allocator> ops::IndexMut<usize> for Slab<'arena, T, A> {
  fn index_mut(&mut self, key: usize) -> &mut T {
    match self.entries.get_mut(key) {
      Some(&mut Entry::Occupied(ref mut v)) => v,
      _ => panic!("invalid key"),
    }
  }
}

impl<'arena, T, A: Allocator> IntoIterator for Slab<'arena, T, A> {
  type Item = (usize, T);
  type IntoIter = IntoIter<T, A>;

  fn into_iter(self) -> IntoIter<T, A> {
    IntoIter {
      entries: self.entries.into_iter().enumerate(),
      len: self.len,
    }
  }
}

impl<'iter, T, A: Allocator> IntoIterator for &'iter Slab<'_, T, A> {
  type Item = (usize, &'iter T);
  type IntoIter = Iter<'iter, T>;

  fn into_iter(self) -> Iter<'iter, T> {
    self.iter()
  }
}

impl<'iter, T, A: Allocator> IntoIterator for &'iter mut Slab<'_, T, A> {
  type Item = (usize, &'iter mut T);
  type IntoIter = IterMut<'iter, T>;

  fn into_iter(self) -> IterMut<'iter, T> {
    self.iter_mut()
  }
}

// impl<'arena, T> FromIterator<(usize, T)> for Slab<'arena, T, &'arena Bump> {
//     fn from_iter<I>(iterable: I) -> Self
//     where
//         I: IntoIterator<Item = (usize, T)>,
//     {
//       let alloc = bumpalo::Bump::new();
//         let iterator = iterable.into_iter();
//         let mut slab = Self::with_capacity_in(iterator.size_hint().0, &alloc);

//         let mut vacant_list_broken = false;
//         let mut first_vacant_index = None;
//         for (key, value) in iterator {
//             if key < slab.entries.len() {
//                 // iterator is not sorted, might need to recreate vacant list
//                 if let Entry::Vacant(_) = slab.entries[key] {
//                     vacant_list_broken = true;
//                     slab.len += 1;
//                 }
//                 // if an element with this key already exists, replace it.
//                 // This is consistent with HashMap and BtreeMap
//                 slab.entries[key] = Entry::Occupied(value);
//             } else {
//                 if first_vacant_index.is_none() && slab.entries.len() < key {
//                     first_vacant_index = Some(slab.entries.len());
//                 }
//                 // insert holes as necessary
//                 while slab.entries.len() < key {
//                     // add the entry to the start of the vacant list
//                     let next = slab.next;
//                     slab.next = slab.entries.len();
//                     slab.entries.push(Entry::Vacant(next));
//                 }
//                 slab.entries.push(Entry::Occupied(value));
//                 slab.len += 1;
//             }
//         }
//         if slab.len == slab.entries.len() {
//             // no vacant entries, so next might not have been updated
//             slab.next = slab.entries.len();
//         } else if vacant_list_broken {
//             slab.recreate_vacant_list();
//         } else if let Some(first_vacant_index) = first_vacant_index {
//             let next = slab.entries.len();
//             match &mut slab.entries[first_vacant_index] {
//                 Entry::Vacant(n) => *n = next,
//                 _ => unreachable!(),
//             }
//         } else {
//             unreachable!()
//         }

//         slab
//     }
// }

impl<T, A: Allocator> fmt::Debug for Slab<'_, T, A>
where
  T: fmt::Debug,
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      fmt.debug_map().entries(self.iter()).finish()
    } else {
      fmt
        .debug_struct("Slab")
        .field("len", &self.len)
        .field("cap", &self.capacity())
        .finish()
    }
  }
}

impl<T, A: Allocator> fmt::Debug for IntoIter<T, A>
where
  T: fmt::Debug,
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("IntoIter")
      .field("remaining", &self.len)
      .finish()
  }
}

impl<T> fmt::Debug for Iter<'_, T>
where
  T: fmt::Debug,
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Iter")
      .field("remaining", &self.len)
      .finish()
  }
}

impl<T> fmt::Debug for IterMut<'_, T>
where
  T: fmt::Debug,
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("IterMut")
      .field("remaining", &self.len)
      .finish()
  }
}

impl<T, A: Allocator> fmt::Debug for Drain<'_, T, A> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_struct("Drain").finish()
  }
}

// ===== VacantEntry =====

impl<'a, T, A: Allocator> VacantEntry<'a, '_, T, A> {
  pub fn insert(self, val: T) -> &'a mut T {
    self.slab.insert_at(self.key, val);

    match self.slab.entries.get_mut(self.key) {
      Some(&mut Entry::Occupied(ref mut v)) => v,
      _ => unreachable!(),
    }
  }

  pub fn key(&self) -> usize {
    self.key
  }
}

// ===== IntoIter =====

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
  type Item = (usize, T);

  fn next(&mut self) -> Option<Self::Item> {
    for (key, entry) in &mut self.entries {
      if let Entry::Occupied(v) = entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
  fn next_back(&mut self) -> Option<Self::Item> {
    while let Some((key, entry)) = self.entries.next_back() {
      if let Entry::Occupied(v) = entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {
  fn len(&self) -> usize {
    self.len
  }
}

impl<T, A: Allocator> FusedIterator for IntoIter<T, A> {}

// ===== Iter =====

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = (usize, &'a T);

  fn next(&mut self) -> Option<Self::Item> {
    for (key, entry) in &mut self.entries {
      if let Entry::Occupied(ref v) = *entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    while let Some((key, entry)) = self.entries.next_back() {
      if let Entry::Occupied(ref v) = *entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
  fn len(&self) -> usize {
    self.len
  }
}

impl<T> FusedIterator for Iter<'_, T> {}

// ===== IterMut =====

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = (usize, &'a mut T);

  fn next(&mut self) -> Option<Self::Item> {
    for (key, entry) in &mut self.entries {
      if let Entry::Occupied(ref mut v) = *entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    while let Some((key, entry)) = self.entries.next_back() {
      if let Entry::Occupied(ref mut v) = *entry {
        self.len -= 1;
        return Some((key, v));
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
  fn len(&self) -> usize {
    self.len
  }
}

impl<T> FusedIterator for IterMut<'_, T> {}

// ===== Drain =====

impl<T, A: Allocator> Iterator for Drain<'_, T, A> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    for entry in &mut self.inner {
      if let Entry::Occupied(v) = entry {
        self.len -= 1;
        return Some(v);
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A> {
  fn next_back(&mut self) -> Option<Self::Item> {
    while let Some(entry) = self.inner.next_back() {
      if let Entry::Occupied(v) = entry {
        self.len -= 1;
        return Some(v);
      }
    }

    debug_assert_eq!(self.len, 0);
    None
  }
}

impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> {
  fn len(&self) -> usize {
    self.len
  }
}

impl<T, A: Allocator> FusedIterator for Drain<'_, T, A> {}
