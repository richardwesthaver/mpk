//! MPK_VM -- ALLOC
use std::alloc::AllocError;
use std::mem::size_of;
use std::ptr::NonNull;

use crate::cons;
use crate::rawptr::RawPtr;

/// A type that describes allocation of an object into a heap space, returning
/// a bare pointer type on success
pub trait AllocRaw {
  /// An implementation of an object header type
  type Header: AllocHeader;

  /// Allocate a single object of type T.
  fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
  where
    T: AllocObject<<Self::Header as AllocHeader>::TypeId>;

  /// Allocating an array allows the client to put anything in the resulting data
  /// block but the type of the memory block will simply be 'Array'. No other
  /// type information will be stored in the object header.
  /// This is just a special case of alloc<T>() for T=u8 but a count > 1 of u8
  /// instances.  The caller is responsible for the content of the array.
  fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError>;

  /// Given a bare pointer to an object, return the expected header address
  fn get_header(object: NonNull<()>) -> NonNull<Self::Header>;

  /// Given a bare pointer to an object's header, return the expected object address
  fn get_object(header: NonNull<Self::Header>) -> NonNull<()>;
}

/// Object size class.
/// - Small objects fit inside a line
/// - Medium objects span more than one line
/// - Large objects span multiple blocks
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SizeClass {
  Small,
  Medium,
  Large,
}

impl SizeClass {
  pub fn get_for_size(object_size: usize) -> Result<SizeClass, AllocError> {
    match object_size {
      cons::SMALL_OBJECT_MIN..=cons::SMALL_OBJECT_MAX => Ok(SizeClass::Small),
      cons::MEDIUM_OBJECT_MIN..=cons::MEDIUM_OBJECT_MAX => Ok(SizeClass::Medium),
      cons::LARGE_OBJECT_MIN..=cons::LARGE_OBJECT_MAX => Ok(SizeClass::Large),
      _ => Err(AllocError),
    }
  }
}

/// The type that describes the bounds of array sizing
pub type ArraySize = u32;

/// TODO Object mark bit.
/// Every object is `Allocated` on creation.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mark {
  Allocated,
  Unmarked,
  Marked,
}

/// A managed-type type-identifier type should implement this!
pub trait AllocTypeId: Copy + Clone {}

/// All managed object types must implement this trait in order to be allocatable
pub trait AllocObject<T: AllocTypeId> {
  const TYPE_ID: T;
}

/// An object header struct must provide an implementation of this trait,
/// providing appropriate information to the garbage collector.
// TODO tracing information
// e.g. fn tracer(&self) -> Fn()
pub trait AllocHeader: Sized {
  /// Associated type that identifies the allocated object type
  type TypeId: AllocTypeId;

  /// Create a new header for object type O
  fn new<O: AllocObject<Self::TypeId>>(
    size: u32,
    size_class: SizeClass,
    mark: Mark,
  ) -> Self;

  /// Create a new header for an array type
  fn new_array(size: ArraySize, size_class: SizeClass, mark: Mark) -> Self;

  /// Set the Mark value to "marked"
  fn mark(&mut self);

  /// Get the current Mark value
  fn is_marked(&self) -> bool;

  /// Get the size class of the object
  fn size_class(&self) -> SizeClass;

  /// Get the size of the object in bytes
  fn size(&self) -> u32;

  /// Get the type of the object
  fn type_id(&self) -> Self::TypeId;
}

/// Return the allocated size of an object as its size_of::<T>() value rounded
/// up to a double-word boundary
///
/// TODO this isn't correctly implemented, as aligning the object to a double-word
/// boundary while considering header size (which is not known to this libarary
/// until compile time) means touching numerous bump-allocation code points with
/// some math and bitwise ops I haven't worked out yet
pub fn alloc_size_of(object_size: usize) -> usize {
  let align = size_of::<usize>(); // * 2;
  (object_size + (align - 1)) & !(align - 1)
}
