//! MPK_VM -- UTIL
use core::sync::atomic::{AtomicPtr, Ordering};
use core::{
  cell::UnsafeCell,
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  ptr,
  ptr::NonNull,
};

pub const fn round_down(x: u64, n: u64) -> u64 {
  x & !n
}

pub const fn round_up(x: u64, n: u64) -> u64 {
  round_down(x + n - 1, n)
}

/// The mask to use for untagging a pointer.
const UNTAG_MASK: usize = (!0x7) as usize;

/// Returns true if the pointer has the given bit set to 1.
pub fn bit_is_set(pointer: u64, bit: usize) -> bool {
  let shifted = 1 << bit;

  (pointer as u64 & shifted) == shifted
}

/// Returns the pointer with the given bit set.
pub fn with_bit(pointer: u64, bit: usize) -> u64 {
  (pointer as u64 | 1 << bit as u64) as _
}

pub fn without_bit(pointer: u64, bit: usize) -> u64 {
  pointer & !(1 << bit)
}

/// Returns the given pointer without any tags set.
pub fn untagged<T>(pointer: u64) -> *mut T {
  (pointer as u64 & UNTAG_MASK as u64) as _
}

/// Structure wrapping a raw, tagged pointer.
#[derive(Debug)]
#[repr(C)]
pub struct TaggedPointer<T> {
  pub raw: u64,
  _marker: PhantomData<T>,
}

impl<T> TaggedPointer<T> {
  /// Returns a new TaggedPointer without setting any bits.
  pub fn new(raw: *mut T) -> TaggedPointer<T> {
    TaggedPointer {
      raw: raw as u64,
      _marker: PhantomData,
    }
  }

  /// Returns a new TaggedPointer with the given bit set.
  pub fn with_bit(raw: *mut T, bit: usize) -> TaggedPointer<T> {
    let mut pointer = Self::new(raw);

    pointer.set_bit(bit);

    pointer
  }

  /// Returns a null pointer.
  pub fn null() -> TaggedPointer<T> {
    TaggedPointer {
      raw: 0,
      _marker: PhantomData,
    }
  }

  /// Returns the wrapped pointer without any tags.
  pub fn untagged(self) -> *mut T {
    self::untagged(self.raw)
  }

  pub fn set_bit_x(&mut self, x: bool, bit: usize) {
    /*if x {
      self.set_bit(bit);
    } else {
      self.clear_bit(bit);
    }*/
    self.raw = self.raw & !(1 << bit as u64) | ((x as u64) << bit as u64);
  }
  pub fn toggle(&mut self, bit: usize) -> bool {
    let x = self.bit_is_set(bit);
    self.raw ^= 1 << bit as u64;

    x
  }
  pub fn clear_bit(&mut self, bit: usize) {
    self.raw = self::without_bit(self.raw, bit);
  }
  /// Returns a new TaggedPointer using the current pointer but without any
  /// tags.
  pub fn without_tags(self) -> Self {
    Self::new(self.untagged())
  }

  /// Returns true if the given bit is set.
  pub fn bit_is_set(self, bit: usize) -> bool {
    self::bit_is_set(self.raw, bit)
  }

  /// Sets the given bit.
  pub fn set_bit(&mut self, bit: usize) {
    self.raw = with_bit(self.raw, bit);
  }

  /// Returns true if the current pointer is a null pointer.
  pub fn is_null(self) -> bool {
    self.untagged().is_null()
  }

  /// Returns an immutable to the pointer's value.
  pub fn as_ref<'a>(self) -> Option<&'a T> {
    unsafe { self.untagged().as_ref() }
  }

  /// Returns a mutable reference to the pointer's value.
  pub fn as_mut<'a>(self) -> Option<&'a mut T> {
    unsafe { self.untagged().as_mut() }
  }

  /// Atomically swaps the internal pointer with another one.
  ///
  /// This boolean returns true if the pointer was swapped, false otherwise.
  #[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
  pub fn compare_and_swap(&self, current: *mut T, other: *mut T) -> bool {
    self.as_atomic().compare_exchange_weak(
      current,
      other,
      Ordering::AcqRel,
      Ordering::Relaxed,
    ) == Ok(current)
  }

  /// Atomically replaces the current pointer with the given one.
  #[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
  pub fn atomic_store(&self, other: *mut T) {
    self.as_atomic().store(other, Ordering::Release);
  }

  /// Atomically loads the pointer.
  #[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
  pub fn atomic_load(&self) -> *mut T {
    self.as_atomic().load(Ordering::Acquire)
  }

  /// Checks if a bit is set using an atomic load.
  #[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
  pub fn atomic_bit_is_set(&self, bit: usize) -> bool {
    Self::new(self.atomic_load()).bit_is_set(bit)
  }

  fn as_atomic(&self) -> &AtomicPtr<T> {
    unsafe { &*(self as *const TaggedPointer<T> as *const AtomicPtr<T>) }
  }
}

impl<T> PartialEq for TaggedPointer<T> {
  fn eq(&self, other: &TaggedPointer<T>) -> bool {
    self.raw == other.raw
  }
}

impl<T> Eq for TaggedPointer<T> {}

// These traits are implemented manually as "derive" doesn't handle the generic
// "T" argument very well.
impl<T> Clone for TaggedPointer<T> {
  fn clone(&self) -> TaggedPointer<T> {
    TaggedPointer::new(self.raw as *mut _)
  }
}

impl<T> Copy for TaggedPointer<T> {}

impl<T> Hash for TaggedPointer<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.raw.hash(state);
  }
}

pub struct FormattedSize {
  size: usize,
}

impl fmt::Display for FormattedSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let ksize = (self.size as f64) / 1024f64;

    if ksize < 1f64 {
      return write!(f, "{}B", self.size);
    }

    let msize = ksize / 1024f64;

    if msize < 1f64 {
      return write!(f, "{:.1}K", ksize);
    }

    let gsize = msize / 1024f64;

    if gsize < 1f64 {
      write!(f, "{:.1}M", msize)
    } else {
      write!(f, "{:.1}G", gsize)
    }
  }
}

pub fn formatted_size(size: usize) -> FormattedSize {
  FormattedSize { size }
}
#[macro_export]
macro_rules! get_sp {
  () => {{
    #[inline(always)]
    #[allow(dead_code)]
    fn generic_get_sp() -> usize {
      let val = 0usize;
      let p = &val as *const usize;
      p as usize
    }

    let sp: usize;
    #[allow(unused_unsafe)]
    unsafe {

      #[cfg(target_arch="x86_64")]
      {

        asm!("mov {},rsp", out(reg) sp,options(nomem,nostack));
      }
      #[cfg(target_arch="x86")]
      {
        asm!("mov {},esp", out(reg) sp,options(nomem,nostack));
      }
      #[cfg(target_arch="aarch64")]
      {
        asm!("add {},sp,#8", out(reg) sp,options(nomem,nostack))
      }
      #[cfg(target_arch="arm")]
      {
        asm!("mov {},sp", out(reg) sp,options(nomem,nostack));
      }
      #[cfg(target_arch="riscv64gc")]
      {
        asm!("add {},zero,sp",out(reg) sp,options(nomem,nostack));
      }
      #[cfg(target_arch="riscv64imac")]
      {
        asm!("add {},zero,sp",out(reg) sp,options(nomem,nostack));
      }
      #[cfg(target_arch="sparc64")]
      {
        sp = generic_get_sp();
      }
      #[cfg(target_arch="powerpc64")]
      {
        sp = generic_get_sp();
      }
      #[cfg(any(target_arch="mipsel",target_arch="mips"))]
      {
        asm!("addiu {},$sp,4",out(reg) sp,options(nomem,nostack));
      }
      #[cfg(any(target_arch="mips64el",target_arch="mips64"))]
      {
        asm!("addiu {},$sp,8",out(reg) sp,options(nomem,nostack));
      }
      sp
    }
  }
  }
}

#[macro_export]
macro_rules! as_atomic {
  ($value: expr;$t: ident) => {
    unsafe { core::mem::transmute::<_, &'_ core::sync::atomic::$t>($value as *const _) }
  };
}

pub unsafe fn zeroed<T>() -> T {
  core::mem::MaybeUninit::<T>::zeroed().assume_init()
}

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
pub struct VolatileCell<T> {
  value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
  /// Creates a new `VolatileCell` containing the given value
  pub const fn new(value: T) -> Self {
    VolatileCell {
      value: UnsafeCell::new(value),
    }
  }

  /// Returns a copy of the contained value
  #[inline(always)]
  pub fn get(&self) -> T
  where
    T: Copy,
  {
    unsafe { ptr::read_volatile(self.value.get()) }
  }

  /// Sets the contained value
  #[inline(always)]
  pub fn set(&self, value: T)
  where
    T: Copy,
  {
    unsafe { ptr::write_volatile(self.value.get(), value) }
  }

  /// Returns a raw pointer to the underlying data in the cell
  #[inline(always)]
  pub fn as_ptr(&self) -> *mut T {
    self.value.get()
  }
}

unsafe impl<T> Sync for VolatileCell<T> {}
unsafe impl<T> Send for VolatileCell<T> {}
