//! Lazy XDR base types: LazyXdr trait, LazyHandle, and primitive implementations.
#![allow(dead_code, unused_imports)]
#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
extern crate alloc;
use alloc::sync::Arc;

// Lazy XDR types: zero-copy access to validated XDR buffers.
//
// This module provides lazy wrappers around XDR-encoded data. Instead of
// eagerly deserializing into in-memory Rust structs, lazy types hold a
// shared-ownership handle to the underlying byte buffer and validate the
// data up front, then provide accessors that compute field offsets on
// demand.

use core::fmt;
use core::marker::PhantomData;

/// Default maximum recursion depth for XDR validation.
pub const DEFAULT_XDR_DEPTH_LIMIT: u32 = 500;

/// A shared-ownership handle to a validated region of an XDR buffer.
#[derive(Clone)]
pub struct LazyHandle {
    buf: Arc<[u8]>,
    pos: u32,
    len: u32,
}

impl PartialEq for LazyHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for LazyHandle {}

impl core::hash::Hash for LazyHandle {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl fmt::Debug for LazyHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LazyHandle")
            .field("pos", &self.pos)
            .field("len", &self.len)
            .finish()
    }
}

impl LazyHandle {
    /// Create a handle covering an entire `Arc<[u8]>` buffer.
    #[must_use]
    pub fn from_arc_complete(buf: Arc<[u8]>) -> Self {
        let len = buf.len() as u32;
        Self { buf, pos: 0, len }
    }

    /// Create a handle from a buffer, position, and length.
    ///
    /// # Panics
    /// Panics if `pos + len` exceeds `buf.len()`.
    #[must_use]
    pub fn from_arc(buf: Arc<[u8]>, pos: u32, len: u32) -> Self {
        assert!((pos as u64) + (len as u64) <= buf.len() as u64);
        Self { buf, pos, len }
    }

    /// Get the validated byte slice this handle refers to.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[self.pos as usize..(self.pos as usize + self.len as usize)]
    }

    /// Create a sub-handle at `offset` within this handle with length `sub_len`.
    ///
    /// This shares ownership of the underlying buffer (cheap `Arc::clone`).
    #[inline]
    #[must_use]
    pub fn sub_handle(&self, offset: u32, sub_len: u32) -> Self {
        Self {
            buf: Arc::clone(&self.buf),
            pos: self.pos + offset,
            len: sub_len,
        }
    }

    /// The length of the validated region in bytes.
    #[inline]
    #[must_use]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Whether this handle covers zero bytes.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get the underlying `Arc<[u8]>`.
    #[inline]
    #[must_use]
    pub fn into_arc(self) -> Arc<[u8]> {
        self.buf
    }
}

impl AsRef<[u8]> for LazyHandle {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

/// Trait for lazy XDR type access over raw byte buffers.
///
/// Types implementing this trait can validate, measure, and extract values
/// from XDR-encoded data without full deserialization.
pub trait LazyXdr: Sized {
    /// The fixed wire size of this type, if statically known.
    /// `None` for variable-length types.
    const FIXED_XDR_SIZE: Option<u32> = None;

    /// Validate XDR data starting at `buf[0..]`.
    ///
    /// Returns the total number of bytes consumed (the wire length) on
    /// success, or an error if the data is invalid or the buffer is too
    /// short.  All arithmetic is checked for overflow.
    ///
    /// The `depth` parameter limits recursion depth to prevent stack overflow
    /// on deeply-nested XDR types.
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error>;

    /// Compute the wire length of already-validated data.
    ///
    /// This must only be called on data that has previously been validated
    /// by [`xdr_validate`](LazyXdr::xdr_validate).  It performs no content
    /// checks and uses plain arithmetic (valid because the data was
    /// validated).
    fn xdr_len(buf: &[u8]) -> u32;

    /// Construct a value by reading from a validated parent handle at the
    /// given byte offset.
    ///
    /// For scalar types this extracts the value directly.
    /// For handle-wrapper types this creates a sub-handle.
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self;
}

// ---------------------------------------------------------------------------
// Primitive LazyXdr implementations
// ---------------------------------------------------------------------------

impl LazyXdr for i32 {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        i32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }
}

impl LazyXdr for u32 {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        u32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }
}

impl LazyXdr for i64 {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 8 {
            return Err(Error::Invalid);
        }
        Ok(8)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        8
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        i64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }
}

impl LazyXdr for u64 {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 8 {
            return Err(Error::Invalid);
        }
        Ok(8)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        8
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }
}

impl LazyXdr for f32 {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        f32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }
}

impl LazyXdr for f64 {
    const FIXED_XDR_SIZE: Option<u32> = Some(8);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 8 {
            return Err(Error::Invalid);
        }
        Ok(8)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        8
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }
}

impl LazyXdr for bool {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let v = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if v > 1 {
            return Err(Error::Invalid);
        }
        Ok(4)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        4
    }

    #[inline]
    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let b = &parent.as_slice()[offset as usize..];
        u32::from_be_bytes([b[0], b[1], b[2], b[3]]) != 0
    }
}

/// () represents XDR void — zero bytes on the wire.
impl LazyXdr for () {
    const FIXED_XDR_SIZE: Option<u32> = Some(0);

    fn xdr_validate(_buf: &[u8], _depth: u32) -> Result<u32, Error> {
        Ok(0)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        0
    }

    #[inline]
    fn from_xdr_at(_parent: &LazyHandle, _offset: u32) -> Self {}
}

// ---------------------------------------------------------------------------
// Built-in lazy wrappers for variable-length XDR types
// ---------------------------------------------------------------------------

/// Padded length: round up to the next multiple of 4.
#[inline]
const fn xdr_pad(n: u32) -> u32 {
    (n + 3) & !3
}

/// Lazy wrapper for XDR `opaque<MAX>` (variable-length opaque data).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyBytesM<const MAX: u32 = { u32::MAX }>(LazyHandle);

impl<const MAX: u32> PartialOrd for LazyBytesM<MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const MAX: u32> Ord for LazyBytesM<MAX> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl<const MAX: u32> LazyXdr for LazyBytesM<MAX> {
    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if len > MAX {
            return Err(Error::LengthExceedsMax);
        }
        let padded = xdr_pad(len);
        let total = 4u32.checked_add(padded).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < total as usize {
            return Err(Error::Invalid);
        }
        // Check padding bytes are zero.
        for &b in &buf[4 + len as usize..total as usize] {
            if b != 0 {
                return Err(Error::NonZeroPadding);
            }
        }
        Ok(total)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        4 + xdr_pad(len)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}

impl<const MAX: u32> LazyBytesM<MAX> {
    /// The data bytes (without the 4-byte length prefix or padding).
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        let buf = self.0.as_slice();
        let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        &buf[4..4 + len]
    }

    /// The number of data bytes.
    #[must_use]
    pub fn data_len(&self) -> u32 {
        u32::from_be_bytes(self.0.as_slice()[0..4].try_into().unwrap())
    }
}

impl<const MAX: u32> AsRef<[u8]> for LazyBytesM<MAX> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const MAX: u32> AsRef<LazyHandle> for LazyBytesM<MAX> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<const MAX: u32> From<LazyHandle> for LazyBytesM<MAX> {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}

/// Lazy wrapper for XDR `string<MAX>` (variable-length string).
///
/// Same wire format as `opaque<MAX>`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStringM<const MAX: u32 = { u32::MAX }>(LazyHandle);

impl<const MAX: u32> PartialOrd for LazyStringM<MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const MAX: u32> Ord for LazyStringM<MAX> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl<const MAX: u32> LazyXdr for LazyStringM<MAX> {
    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        // String has the same wire format as opaque.
        LazyBytesM::<MAX>::xdr_validate(buf, _depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        LazyBytesM::<MAX>::xdr_len(buf)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}

impl<const MAX: u32> LazyStringM<MAX> {
    /// The string bytes (without the 4-byte length prefix or padding).
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        let buf = self.0.as_slice();
        let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        &buf[4..4 + len]
    }

    /// The number of string bytes.
    #[must_use]
    pub fn data_len(&self) -> u32 {
        u32::from_be_bytes(self.0.as_slice()[0..4].try_into().unwrap())
    }

    /// Try to interpret the string bytes as UTF-8.
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }
}

impl<const MAX: u32> AsRef<[u8]> for LazyStringM<MAX> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const MAX: u32> AsRef<LazyHandle> for LazyStringM<MAX> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<const MAX: u32> From<LazyHandle> for LazyStringM<MAX> {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}

/// Lazy wrapper for XDR variable-length arrays `T<MAX>`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyVecM<T: LazyXdr, const MAX: u32 = { u32::MAX }>(LazyHandle, PhantomData<fn() -> T>);

impl<T: LazyXdr, const MAX: u32> LazyXdr for LazyVecM<T, MAX> {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let count = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if count > MAX {
            return Err(Error::LengthExceedsMax);
        }
        let mut pos: u32 = 4;
        for _ in 0..count {
            let elem_len = T::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(elem_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let count = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            4 + count * fixed
        } else {
            let mut pos: u32 = 4;
            for _ in 0..count {
                pos += T::xdr_len(&buf[pos as usize..]);
            }
            pos
        }
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len), PhantomData)
    }
}

impl<T: LazyXdr, const MAX: u32> LazyVecM<T, MAX> {
    /// The number of elements.
    #[must_use]
    pub fn element_count(&self) -> u32 {
        u32::from_be_bytes(self.0.as_slice()[0..4].try_into().unwrap())
    }

    /// Access the element at `index` (0-based).
    /// Returns `None` if `index >= self.element_count()`.
    #[must_use]
    pub fn get(&self, index: u32) -> Option<T> {
        let buf = self.0.as_slice();
        let count = self.element_count();
        if index >= count {
            return None;
        }
        let mut pos: u32 = 4;
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            pos += index * fixed;
        } else {
            for _ in 0..index {
                pos += T::xdr_len(&buf[pos as usize..]);
            }
        }
        Some(T::from_xdr_at(&self.0, pos))
    }

    /// Return a forward-scanning iterator over all elements.
    ///
    /// This is O(n) for the full traversal, unlike calling `get(i)` in a
    /// loop which is O(n²) for variable-length element types.
    #[must_use]
    pub fn iter(&self) -> LazyVecMIter<'_, T, MAX> {
        LazyVecMIter {
            handle: &self.0,
            index: 0,
            pos: 4,
            count: self.element_count(),
            _marker: PhantomData,
        }
    }
}

/// Forward-scanning iterator over [`LazyVecM`] elements.
pub struct LazyVecMIter<'a, T: LazyXdr, const MAX: u32 = { u32::MAX }> {
    handle: &'a LazyHandle,
    index: u32,
    pos: u32,
    count: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T: LazyXdr, const MAX: u32> Iterator for LazyVecMIter<'a, T, MAX> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index >= self.count {
            return None;
        }
        let elem = T::from_xdr_at(self.handle, self.pos);
        let buf = self.handle.as_slice();
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            self.pos += fixed;
        } else {
            self.pos += T::xdr_len(&buf[self.pos as usize..]);
        }
        self.index += 1;
        Some(elem)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let r = (self.count - self.index) as usize;
        (r, Some(r))
    }
}

impl<T: LazyXdr, const MAX: u32> ExactSizeIterator for LazyVecMIter<'_, T, MAX> {}

impl<T: LazyXdr + Ord, const MAX: u32> PartialOrd for LazyVecM<T, MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: LazyXdr + Ord, const MAX: u32> Ord for LazyVecM<T, MAX> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: LazyXdr, const MAX: u32> AsRef<LazyHandle> for LazyVecM<T, MAX> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<T: LazyXdr, const MAX: u32> From<LazyHandle> for LazyVecM<T, MAX> {
    fn from(h: LazyHandle) -> Self {
        Self(h, PhantomData)
    }
}

/// Lazy wrapper for XDR fixed-length arrays `T[N]`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyFixedArray<T: LazyXdr, const N: u32>(LazyHandle, PhantomData<fn() -> T>);

impl<T: LazyXdr, const N: u32> LazyXdr for LazyFixedArray<T, N> {
    const FIXED_XDR_SIZE: Option<u32> = match T::FIXED_XDR_SIZE {
        Some(elem) => Some(elem * N),
        None => None,
    };

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        let mut pos: u32 = 0;
        for _ in 0..N {
            let elem_len = T::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(elem_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            fixed * N
        } else {
            let mut pos: u32 = 0;
            for _ in 0..N {
                pos += T::xdr_len(&buf[pos as usize..]);
            }
            pos
        }
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len), PhantomData)
    }
}

impl<T: LazyXdr, const N: u32> LazyFixedArray<T, N> {
    /// Access the element at `index` (0-based).
    /// Returns `None` if `index >= N`.
    #[must_use]
    pub fn get(&self, index: u32) -> Option<T> {
        if index >= N {
            return None;
        }
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            pos = index * fixed;
        } else {
            for _ in 0..index {
                pos += T::xdr_len(&buf[pos as usize..]);
            }
        }
        Some(T::from_xdr_at(&self.0, pos))
    }

    /// The number of elements (always `N`).
    #[must_use]
    pub const fn element_count(&self) -> u32 {
        N
    }

    /// Return a forward-scanning iterator over all elements.
    #[must_use]
    pub fn iter(&self) -> LazyFixedArrayIter<'_, T, N> {
        LazyFixedArrayIter {
            handle: &self.0,
            index: 0,
            pos: 0,
            _marker: PhantomData,
        }
    }
}

/// Forward-scanning iterator over [`LazyFixedArray`] elements.
pub struct LazyFixedArrayIter<'a, T: LazyXdr, const N: u32> {
    handle: &'a LazyHandle,
    index: u32,
    pos: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T: LazyXdr, const N: u32> Iterator for LazyFixedArrayIter<'a, T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index >= N {
            return None;
        }
        let elem = T::from_xdr_at(self.handle, self.pos);
        let buf = self.handle.as_slice();
        if let Some(fixed) = T::FIXED_XDR_SIZE {
            self.pos += fixed;
        } else {
            self.pos += T::xdr_len(&buf[self.pos as usize..]);
        }
        self.index += 1;
        Some(elem)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let r = (N - self.index) as usize;
        (r, Some(r))
    }
}

impl<T: LazyXdr, const N: u32> ExactSizeIterator for LazyFixedArrayIter<'_, T, N> {}

impl<T: LazyXdr + Ord, const N: u32> PartialOrd for LazyFixedArray<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: LazyXdr + Ord, const N: u32> Ord for LazyFixedArray<T, N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: LazyXdr, const N: u32> AsRef<LazyHandle> for LazyFixedArray<T, N> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<T: LazyXdr, const N: u32> From<LazyHandle> for LazyFixedArray<T, N> {
    fn from(h: LazyHandle) -> Self {
        Self(h, PhantomData)
    }
}

/// Lazy wrapper for XDR `opaque[N]` (fixed-length opaque data).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOpaqueFixed<const N: u32>(LazyHandle);

impl<const N: u32> PartialOrd for LazyOpaqueFixed<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: u32> Ord for LazyOpaqueFixed<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl<const N: u32> LazyXdr for LazyOpaqueFixed<N> {
    const FIXED_XDR_SIZE: Option<u32> = Some(xdr_pad(N));

    fn xdr_validate(buf: &[u8], _depth: u32) -> Result<u32, Error> {
        let padded = xdr_pad(N);
        if buf.len() < padded as usize {
            return Err(Error::Invalid);
        }
        // Check padding bytes are zero.
        for &b in &buf[N as usize..padded as usize] {
            if b != 0 {
                return Err(Error::NonZeroPadding);
            }
        }
        Ok(padded)
    }

    #[inline]
    fn xdr_len(_buf: &[u8]) -> u32 {
        xdr_pad(N)
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let len = xdr_pad(N);
        Self(parent.sub_handle(offset, len))
    }
}

impl<const N: u32> LazyOpaqueFixed<N> {
    /// The data bytes (without padding).
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_slice()[..N as usize]
    }
}

impl<const N: u32> AsRef<[u8]> for LazyOpaqueFixed<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: u32> AsRef<LazyHandle> for LazyOpaqueFixed<N> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<const N: u32> From<LazyHandle> for LazyOpaqueFixed<N> {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}

/// Lazy wrapper for XDR optional `T*`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOption<T: LazyXdr>(LazyHandle, PhantomData<fn() -> T>);

impl<T: LazyXdr + Ord> PartialOrd for LazyOption<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: LazyXdr + Ord> Ord for LazyOption<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: LazyXdr> LazyXdr for LazyOption<T> {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        match disc {
            0 => Ok(4),
            1 => {
                let inner_len = T::xdr_validate(&buf[4..], depth)?;
                4u32.checked_add(inner_len).ok_or(Error::LengthExceedsMax)
            }
            _ => Err(Error::Invalid),
        }
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let disc = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if disc == 0 {
            4
        } else {
            4 + T::xdr_len(&buf[4..])
        }
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len), PhantomData)
    }
}

impl<T: LazyXdr> LazyOption<T> {
    /// Whether the optional value is present.
    #[must_use]
    pub fn is_some(&self) -> bool {
        let buf = self.0.as_slice();
        u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) != 0
    }

    /// Whether the optional value is absent.
    #[must_use]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Get the inner value, if present.
    #[must_use]
    pub fn get(&self) -> Option<T> {
        if self.is_some() {
            Some(T::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}

impl<T: LazyXdr> AsRef<LazyHandle> for LazyOption<T> {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}

impl<T: LazyXdr> From<LazyHandle> for LazyOption<T> {
    fn from(h: LazyHandle) -> Self {
        Self(h, PhantomData)
    }
}
