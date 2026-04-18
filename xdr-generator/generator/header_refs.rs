// Zero-copy XDR buffer reference infrastructure.
//
// This module provides `XdrRef<T>`, a 64-bit (offset + length) reference into
// an XDR-encoded byte buffer. After a one-time validation pass, sub-field
// accessors can navigate the buffer without allocations.

// Note: `fmt`, `PhantomData` are already imported by header.rs.
use core::cmp::Ordering;

/// `RefLimits` contains the limits enforced during XDR buffer validation.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RefLimits {
    /// Maximum remaining recursion depth.
    pub depth: u32,
}

impl RefLimits {
    /// Constructs limits with the given depth budget.
    #[must_use]
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }

    /// Constructs limits with no depth restriction.
    #[must_use]
    pub fn none() -> Self {
        Self { depth: u32::MAX }
    }

    /// Consumes a single depth level for the duration of the given function.
    ///
    /// ### Errors
    ///
    /// Returns [`Error::DepthLimitExceeded`] if the depth budget is exhausted.
    pub fn with_limited_depth<T, F>(&mut self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Self) -> Result<T, Error>,
    {
        if let Some(depth) = self.depth.checked_sub(1) {
            self.depth = depth;
            let res = f(self);
            self.depth = self.depth.saturating_add(1);
            res
        } else {
            Err(Error::DepthLimitExceeded)
        }
    }
}

// ---------------------------------------------------------------------------
// XdrRef<T> — a 64-bit (offset, len) reference into an XDR buffer.
// ---------------------------------------------------------------------------

/// A zero-copy reference into an XDR-encoded byte buffer.
///
/// `XdrRef<T>` stores only an offset and length (8 bytes total). It is `Copy`,
/// `Send`, and `Sync`. The type parameter `T` identifies which XDR type the
/// referenced bytes encode, but `XdrRef` never owns a `T`.
///
/// # Typical workflow
///
/// 1. **Validate** once via a generated `validate(buf, offset, limits)` function
///    to obtain an `XdrRef<T>`.
/// 2. **Access** sub-fields using the generated accessor methods, which return
///    further `XdrRef`s or decoded primitive values.
/// 3. **Materialize** into an owned `T` when the full decoded value is needed.
/// 4. **Compare** two refs for equality or ordering without materializing.
///
/// # Safety note
///
/// Accessors perform bounds checks and return `Result`. They never panic.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct XdrRef<T> {
    offset: u32,
    len: u32,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for XdrRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdrRef")
            .field("offset", &self.offset)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> XdrRef<T> {
    /// Construct an `XdrRef` from a validated offset and length.
    ///
    /// This is intended for internal use by generated `validate` functions.
    #[inline]
    #[must_use]
    pub const fn new(offset: u32, len: u32) -> Self {
        Self {
            offset,
            len,
            _phantom: PhantomData,
        }
    }

    /// Byte offset where this value starts in the buffer.
    #[inline]
    #[must_use]
    pub const fn offset(&self) -> u32 {
        self.offset
    }

    /// Total byte length of the XDR encoding (including any padding).
    #[inline]
    #[must_use]
    pub const fn len(&self) -> u32 {
        self.len
    }

    /// One past the last byte: `offset() + len()`.
    #[inline]
    #[must_use]
    pub const fn end(&self) -> u32 {
        self.offset + self.len
    }

    /// Returns the raw XDR bytes for this value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Invalid`] if the buffer is too short.
    #[inline]
    pub fn as_slice<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        let start = self.offset as usize;
        let end = start
            .checked_add(self.len as usize)
            .ok_or(Error::LengthExceedsMax)?;
        if end > buf.len() {
            return Err(Error::Invalid);
        }
        Ok(&buf[start..end])
    }

    /// Byte-slice equality of the XDR encoding. Valid because XDR has a
    /// canonical encoding after validation.
    #[inline]
    pub fn eq_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> bool {
        match (self.as_slice(buf), other.as_slice(other_buf)) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
    }

    /// Reinterpret this ref as a ref to a different type `U` with the same
    /// offset and length. Used for transparent newtypes (typedefs).
    #[inline]
    #[must_use]
    pub const fn cast<U>(self) -> XdrRef<U> {
        XdrRef::new(self.offset, self.len)
    }
}

// ---------------------------------------------------------------------------
// Bounds-checked buffer read helpers
// ---------------------------------------------------------------------------

/// Read a big-endian `u32` at the given byte offset.
#[inline]
pub fn read_u32_at(buf: &[u8], offset: u32) -> Result<u32, Error> {
    let off = offset as usize;
    let end = off.checked_add(4).ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        return Err(Error::Invalid);
    }
    let b: [u8; 4] = buf[off..end].try_into().map_err(|_| Error::Invalid)?;
    Ok(u32::from_be_bytes(b))
}

/// Read a big-endian `i32` at the given byte offset.
#[inline]
pub fn read_i32_at(buf: &[u8], offset: u32) -> Result<i32, Error> {
    let off = offset as usize;
    let end = off.checked_add(4).ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        return Err(Error::Invalid);
    }
    let b: [u8; 4] = buf[off..end].try_into().map_err(|_| Error::Invalid)?;
    Ok(i32::from_be_bytes(b))
}

/// Read a big-endian `u64` at the given byte offset.
#[inline]
pub fn read_u64_at(buf: &[u8], offset: u32) -> Result<u64, Error> {
    let off = offset as usize;
    let end = off.checked_add(8).ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        return Err(Error::Invalid);
    }
    let b: [u8; 8] = buf[off..end].try_into().map_err(|_| Error::Invalid)?;
    Ok(u64::from_be_bytes(b))
}

/// Read a big-endian `i64` at the given byte offset.
#[inline]
pub fn read_i64_at(buf: &[u8], offset: u32) -> Result<i64, Error> {
    let off = offset as usize;
    let end = off.checked_add(8).ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        return Err(Error::Invalid);
    }
    let b: [u8; 8] = buf[off..end].try_into().map_err(|_| Error::Invalid)?;
    Ok(i64::from_be_bytes(b))
}

/// Read an XDR bool (4-byte big-endian, 0 or 1) at the given byte offset.
#[inline]
pub fn read_bool_at(buf: &[u8], offset: u32) -> Result<bool, Error> {
    let v = read_u32_at(buf, offset)?;
    match v {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::Invalid),
    }
}

/// Return a byte slice of length `len` starting at `offset`.
#[inline]
pub fn read_bytes_at(buf: &[u8], offset: u32, len: u32) -> Result<&[u8], Error> {
    let off = offset as usize;
    let end = off
        .checked_add(len as usize)
        .ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        return Err(Error::Invalid);
    }
    Ok(&buf[off..end])
}

/// Compute XDR padding length for a data length.
#[inline]
#[must_use]
pub const fn pad_len_ref(len: u32) -> u32 {
    (4u32.wrapping_sub(len % 4)) % 4
}

/// Check that the padding bytes at `offset` are all zero.
#[inline]
pub fn check_padding(buf: &[u8], offset: u32, pad: u32) -> Result<(), Error> {
    if pad == 0 {
        return Ok(());
    }
    let bytes = read_bytes_at(buf, offset, pad)?;
    if bytes.iter().any(|&b| b != 0) {
        return Err(Error::NonZeroPadding);
    }
    Ok(())
}

/// Check that `offset + additional` does not exceed `buf.len()`.
#[inline]
pub fn check_bounds(buf: &[u8], offset: u32, additional: u32) -> Result<(), Error> {
    let end = (offset as usize)
        .checked_add(additional as usize)
        .ok_or(Error::LengthExceedsMax)?;
    if end > buf.len() {
        Err(Error::Invalid)
    } else {
        Ok(())
    }
}

/// Compute the XDR-encoded size of a variable-length opaque/string/bytes value
/// starting at `offset` (which points to the 4-byte length prefix). Returns
/// `(data_len, total_xdr_len)` where `total_xdr_len` includes the 4-byte
/// length prefix, the data, and padding.
#[inline]
pub fn var_opaque_xdr_len(buf: &[u8], offset: u32, max: u32) -> Result<(u32, u32), Error> {
    let data_len = read_u32_at(buf, offset)?;
    if data_len > max {
        return Err(Error::LengthExceedsMax);
    }
    let pad = pad_len_ref(data_len);
    let total = 4u32
        .checked_add(data_len)
        .and_then(|v| v.checked_add(pad))
        .ok_or(Error::LengthExceedsMax)?;
    // Check bounds for data + padding.
    check_bounds(buf, offset, total)?;
    // Verify padding is zero.
    let pad_offset = offset
        .checked_add(4)
        .and_then(|v| v.checked_add(data_len))
        .ok_or(Error::LengthExceedsMax)?;
    check_padding(buf, pad_offset, pad)?;
    Ok((data_len, total))
}

// ---------------------------------------------------------------------------
// Validate + accessors for primitive types
// ---------------------------------------------------------------------------

/// Validate an XDR `i32` at the given offset. Returns a ref of length 4.
#[inline]
pub fn validate_i32(buf: &[u8], offset: u32) -> Result<XdrRef<i32>, Error> {
    check_bounds(buf, offset, 4)?;
    Ok(XdrRef::new(offset, 4))
}

/// Validate an XDR `u32` at the given offset. Returns a ref of length 4.
#[inline]
pub fn validate_u32(buf: &[u8], offset: u32) -> Result<XdrRef<u32>, Error> {
    check_bounds(buf, offset, 4)?;
    Ok(XdrRef::new(offset, 4))
}

/// Validate an XDR `i64` at the given offset. Returns a ref of length 8.
#[inline]
pub fn validate_i64(buf: &[u8], offset: u32) -> Result<XdrRef<i64>, Error> {
    check_bounds(buf, offset, 8)?;
    Ok(XdrRef::new(offset, 8))
}

/// Validate an XDR `u64` at the given offset. Returns a ref of length 8.
#[inline]
pub fn validate_u64(buf: &[u8], offset: u32) -> Result<XdrRef<u64>, Error> {
    check_bounds(buf, offset, 8)?;
    Ok(XdrRef::new(offset, 8))
}

/// Validate an XDR `bool` at the given offset. Returns a ref of length 4.
#[inline]
pub fn validate_bool(buf: &[u8], offset: u32) -> Result<XdrRef<bool>, Error> {
    check_bounds(buf, offset, 4)?;
    // Validate that it is 0 or 1.
    let _ = read_bool_at(buf, offset)?;
    Ok(XdrRef::new(offset, 4))
}

/// Validate an XDR `()` (void). Always succeeds with length 0.
#[inline]
pub fn validate_void(_buf: &[u8], offset: u32) -> Result<XdrRef<()>, Error> {
    Ok(XdrRef::new(offset, 0))
}

// Accessor impls for primitives.
impl XdrRef<i32> {
    pub const XDR_FIXED_SIZE: u32 = 4;

    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<i32, Error> {
        read_i32_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.get(buf)?.cmp(&other.get(other_buf)?))
    }
}

impl XdrRef<u32> {
    pub const XDR_FIXED_SIZE: u32 = 4;

    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<u32, Error> {
        read_u32_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.get(buf)?.cmp(&other.get(other_buf)?))
    }
}

impl XdrRef<i64> {
    pub const XDR_FIXED_SIZE: u32 = 8;

    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<i64, Error> {
        read_i64_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.get(buf)?.cmp(&other.get(other_buf)?))
    }
}

impl XdrRef<u64> {
    pub const XDR_FIXED_SIZE: u32 = 8;

    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<u64, Error> {
        read_u64_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.get(buf)?.cmp(&other.get(other_buf)?))
    }
}

impl XdrRef<bool> {
    pub const XDR_FIXED_SIZE: u32 = 4;

    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<bool, Error> {
        read_bool_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.get(buf)?.cmp(&other.get(other_buf)?))
    }
}

impl XdrRef<()> {
    pub const XDR_FIXED_SIZE: u32 = 0;
}

// ---------------------------------------------------------------------------
// Fixed-size opaque: [u8; N]
// ---------------------------------------------------------------------------

/// Validate an XDR fixed-size opaque `opaque[N]` at the given offset.
/// The encoded size is `N + pad_len(N)`.
#[inline]
pub fn validate_fixed_opaque<const N: usize>(
    buf: &[u8],
    offset: u32,
) -> Result<XdrRef<[u8; N]>, Error> {
    let n = N as u32;
    let pad = pad_len_ref(n);
    let total = n.checked_add(pad).ok_or(Error::LengthExceedsMax)?;
    check_bounds(buf, offset, total)?;
    let pad_offset = offset.checked_add(n).ok_or(Error::LengthExceedsMax)?;
    check_padding(buf, pad_offset, pad)?;
    Ok(XdrRef::new(offset, total))
}

impl<const N: usize> XdrRef<[u8; N]> {
    /// Returns the data bytes (without padding).
    #[inline]
    pub fn as_bytes<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        read_bytes_at(buf, self.offset, N as u32)
    }

    /// Returns the data as a fixed-size array.
    #[inline]
    pub fn get(&self, buf: &[u8]) -> Result<[u8; N], Error> {
        let bytes = self.as_bytes(buf)?;
        let arr: [u8; N] = bytes.try_into().map_err(|_| Error::Invalid)?;
        Ok(arr)
    }
}

// ---------------------------------------------------------------------------
// Option<T>
// ---------------------------------------------------------------------------

/// Validate an XDR optional value at the given offset.
///
/// XDR optional: 4-byte flag (0 or 1), followed by T if 1.
///
/// `validate_inner` is a function that validates the inner type T starting at
/// a given offset, returning the XDR length of T.
#[inline]
pub fn validate_option<T>(
    buf: &[u8],
    offset: u32,
    validate_inner: impl FnOnce(&[u8], u32) -> Result<u32, Error>,
) -> Result<XdrRef<Option<T>>, Error> {
    let flag = read_u32_at(buf, offset)?;
    match flag {
        0 => Ok(XdrRef::new(offset, 4)),
        1 => {
            let inner_offset = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;
            let inner_len = validate_inner(buf, inner_offset)?;
            let total = 4u32
                .checked_add(inner_len)
                .ok_or(Error::LengthExceedsMax)?;
            Ok(XdrRef::new(offset, total))
        }
        _ => Err(Error::Invalid),
    }
}

impl<T> XdrRef<Option<T>> {
    /// Returns whether the optional value is present.
    #[inline]
    pub fn is_some(&self, buf: &[u8]) -> Result<bool, Error> {
        let flag = read_u32_at(buf, self.offset)?;
        Ok(flag == 1)
    }

    /// Returns a ref to the inner value, or `None` if the optional is absent.
    ///
    /// The caller must provide `inner_len` for variable-size inner types. For
    /// fixed-size inner types, use `inner_fixed` instead.
    #[inline]
    pub fn inner(&self, buf: &[u8]) -> Result<Option<XdrRef<T>>, Error> {
        let flag = read_u32_at(buf, self.offset)?;
        match flag {
            0 => Ok(None),
            1 => {
                let inner_offset = self
                    .offset
                    .checked_add(4)
                    .ok_or(Error::LengthExceedsMax)?;
                let inner_len = self.len.checked_sub(4).ok_or(Error::Invalid)?;
                Ok(Some(XdrRef::new(inner_offset, inner_len)))
            }
            _ => Err(Error::Invalid),
        }
    }

    /// Compare two optional refs by XDR byte content.
    /// Canonical XDR makes byte comparison correct after validation.
    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        let a = self.as_slice(buf)?;
        let b = other.as_slice(other_buf)?;
        Ok(a.cmp(b))
    }
}

// ---------------------------------------------------------------------------
// Box<T> — transparent delegation
// ---------------------------------------------------------------------------

/// Validate an XDR `Box<T>` at the given offset. Transparent: identical to
/// validating `T`.
#[inline]
pub fn validate_box<T>(
    buf: &[u8],
    offset: u32,
    validate_inner: impl FnOnce(&[u8], u32) -> Result<u32, Error>,
) -> Result<XdrRef<Box<T>>, Error> {
    let inner_len = validate_inner(buf, offset)?;
    Ok(XdrRef::new(offset, inner_len))
}

impl<T> XdrRef<Box<T>> {
    /// Returns a ref to the inner value (same offset and length).
    #[inline]
    #[must_use]
    pub const fn inner(self) -> XdrRef<T> {
        XdrRef::new(self.offset, self.len)
    }

    /// Compare two boxed refs by XDR byte content.
    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        let a = self.as_slice(buf)?;
        let b = other.as_slice(other_buf)?;
        Ok(a.cmp(b))
    }
}

// ---------------------------------------------------------------------------
// VecM<T, MAX> — variable-length arrays
// ---------------------------------------------------------------------------

/// Validate an XDR variable-length array of elements at the given offset.
///
/// `validate_element` validates one element at a given offset, returning the
/// element's XDR byte length. For fixed-size elements, use
/// `validate_vec_fixed` instead.
pub fn validate_vec<T, const MAX: u32>(
    buf: &[u8],
    offset: u32,
    limits: &mut RefLimits,
    validate_element: impl Fn(&[u8], u32, &mut RefLimits) -> Result<u32, Error>,
) -> Result<XdrRef<VecM<T, MAX>>, Error> {
    let count = read_u32_at(buf, offset)?;
    if count > MAX {
        return Err(Error::LengthExceedsMax);
    }
    let mut pos = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;
    for _ in 0..count {
        let elem_len = validate_element(buf, pos, limits)?;
        pos = pos.checked_add(elem_len).ok_or(Error::LengthExceedsMax)?;
    }
    let total = pos.checked_sub(offset).ok_or(Error::Invalid)?;
    Ok(XdrRef::new(offset, total))
}

/// Validate an XDR variable-length array of fixed-size elements.
pub fn validate_vec_fixed<T, const MAX: u32>(
    buf: &[u8],
    offset: u32,
    elem_size: u32,
) -> Result<XdrRef<VecM<T, MAX>>, Error> {
    let count = read_u32_at(buf, offset)?;
    if count > MAX {
        return Err(Error::LengthExceedsMax);
    }
    let data_len = count
        .checked_mul(elem_size)
        .ok_or(Error::LengthExceedsMax)?;
    let total = 4u32
        .checked_add(data_len)
        .ok_or(Error::LengthExceedsMax)?;
    check_bounds(buf, offset, total)?;
    Ok(XdrRef::new(offset, total))
}

/// Validate an XDR variable-length array of bytes (`VecM<u8, MAX>`).
/// These have the same encoding as `BytesM<MAX>`: 4-byte len + data + padding.
pub fn validate_vec_u8<const MAX: u32>(
    buf: &[u8],
    offset: u32,
) -> Result<XdrRef<VecM<u8, MAX>>, Error> {
    let (_, total) = var_opaque_xdr_len(buf, offset, MAX)?;
    Ok(XdrRef::new(offset, total))
}

impl<T, const MAX: u32> XdrRef<VecM<T, MAX>> {
    /// Returns the element count.
    #[inline]
    pub fn count(&self, buf: &[u8]) -> Result<u32, Error> {
        read_u32_at(buf, self.offset)
    }

    /// Compare two vec refs by XDR byte content.
    /// Canonical XDR makes byte comparison correct after validation.
    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        let a = self.as_slice(buf)?;
        let b = other.as_slice(other_buf)?;
        Ok(a.cmp(b))
    }
}

/// Iterator over elements of a validated `XdrRef<VecM<T, MAX>>`.
pub struct VecRefIter<'a, T, const MAX: u32> {
    buf: &'a [u8],
    remaining: u32,
    pos: u32,
    _phantom: PhantomData<fn() -> T>,
}

// ---------------------------------------------------------------------------
// BytesM<MAX>
// ---------------------------------------------------------------------------

/// Validate an XDR `BytesM<MAX>` at the given offset.
pub fn validate_bytes<const MAX: u32>(
    buf: &[u8],
    offset: u32,
) -> Result<XdrRef<BytesM<MAX>>, Error> {
    let (_, total) = var_opaque_xdr_len(buf, offset, MAX)?;
    Ok(XdrRef::new(offset, total))
}

impl<const MAX: u32> XdrRef<BytesM<MAX>> {
    /// Returns the data bytes (without length prefix or padding).
    #[inline]
    pub fn as_bytes<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        let data_len = read_u32_at(buf, self.offset)?;
        let data_offset = self
            .offset
            .checked_add(4)
            .ok_or(Error::LengthExceedsMax)?;
        read_bytes_at(buf, data_offset, data_len)
    }

    /// Returns the data byte count.
    #[inline]
    pub fn count(&self, buf: &[u8]) -> Result<u32, Error> {
        read_u32_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.as_bytes(buf)?.cmp(other.as_bytes(other_buf)?))
    }
}

// ---------------------------------------------------------------------------
// StringM<MAX>
// ---------------------------------------------------------------------------

/// Validate an XDR `StringM<MAX>` at the given offset.
pub fn validate_string<const MAX: u32>(
    buf: &[u8],
    offset: u32,
) -> Result<XdrRef<StringM<MAX>>, Error> {
    let (_, total) = var_opaque_xdr_len(buf, offset, MAX)?;
    Ok(XdrRef::new(offset, total))
}

impl<const MAX: u32> XdrRef<StringM<MAX>> {
    /// Returns the string data bytes (without length prefix or padding).
    #[inline]
    pub fn as_bytes<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        let data_len = read_u32_at(buf, self.offset)?;
        let data_offset = self
            .offset
            .checked_add(4)
            .ok_or(Error::LengthExceedsMax)?;
        read_bytes_at(buf, data_offset, data_len)
    }

    /// Returns the data byte count.
    #[inline]
    pub fn count(&self, buf: &[u8]) -> Result<u32, Error> {
        read_u32_at(buf, self.offset)
    }

    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        Ok(self.as_bytes(buf)?.cmp(other.as_bytes(other_buf)?))
    }
}

// ---------------------------------------------------------------------------
// Fixed-size arrays [T; N] (non-opaque)
// ---------------------------------------------------------------------------

/// Validate an XDR fixed-size array `[T; N]` at the given offset, where each
/// element has a fixed XDR size of `elem_size`.
pub fn validate_fixed_array<T, const N: usize>(
    buf: &[u8],
    offset: u32,
    elem_size: u32,
) -> Result<XdrRef<[T; N]>, Error> {
    let total = (N as u32)
        .checked_mul(elem_size)
        .ok_or(Error::LengthExceedsMax)?;
    check_bounds(buf, offset, total)?;
    Ok(XdrRef::new(offset, total))
}

/// Validate an XDR fixed-size array `[T; N]` of variable-size elements.
pub fn validate_fixed_array_var<T, const N: usize>(
    buf: &[u8],
    offset: u32,
    limits: &mut RefLimits,
    validate_element: impl Fn(&[u8], u32, &mut RefLimits) -> Result<u32, Error>,
) -> Result<XdrRef<[T; N]>, Error> {
    let mut pos = offset;
    for _ in 0..N {
        let elem_len = validate_element(buf, pos, limits)?;
        pos = pos.checked_add(elem_len).ok_or(Error::LengthExceedsMax)?;
    }
    let total = pos.checked_sub(offset).ok_or(Error::Invalid)?;
    Ok(XdrRef::new(offset, total))
}

impl<T, const N: usize> XdrRef<[T; N]> {
    /// Returns a ref to the i-th element in a fixed-size array of fixed-size elements.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Invalid`] if `i >= N`.
    #[inline]
    pub fn get_fixed(&self, i: usize, elem_size: u32) -> Result<XdrRef<T>, Error> {
        if i >= N {
            return Err(Error::Invalid);
        }
        let elem_offset = self
            .offset
            .checked_add((i as u32).checked_mul(elem_size).ok_or(Error::LengthExceedsMax)?)
            .ok_or(Error::LengthExceedsMax)?;
        Ok(XdrRef::new(elem_offset, elem_size))
    }

    /// Compare two fixed-array refs by XDR byte content.
    #[inline]
    pub fn cmp_in(&self, buf: &[u8], other: &Self, other_buf: &[u8]) -> Result<Ordering, Error> {
        let a = self.as_slice(buf)?;
        let b = other.as_slice(other_buf)?;
        Ok(a.cmp(b))
    }
}

// ---------------------------------------------------------------------------
// Skip helpers — advance past a single field of a given type
// ---------------------------------------------------------------------------

/// Skip past an XDR variable-length opaque/bytes/string (4-byte length prefix +
/// data + padding). Returns the total number of bytes consumed.
#[inline]
pub fn skip_var_opaque(buf: &[u8], offset: u32, max: u32) -> Result<u32, Error> {
    let (_, total) = var_opaque_xdr_len(buf, offset, max)?;
    Ok(total)
}

/// Skip past an XDR `Option<T>`. Returns total bytes consumed.
///
/// `skip_inner` should return the XDR byte length of the inner T.
#[inline]
pub fn skip_option(
    buf: &[u8],
    offset: u32,
    skip_inner: impl FnOnce(&[u8], u32) -> Result<u32, Error>,
) -> Result<u32, Error> {
    let flag = read_u32_at(buf, offset)?;
    match flag {
        0 => Ok(4),
        1 => {
            let inner_offset = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;
            let inner_len = skip_inner(buf, inner_offset)?;
            4u32.checked_add(inner_len).ok_or(Error::LengthExceedsMax)
        }
        _ => Err(Error::Invalid),
    }
}

/// Skip past an XDR variable-length array of fixed-size elements.
/// Returns total bytes consumed (4-byte count + count * elem_size).
#[inline]
pub fn skip_vec_fixed(buf: &[u8], offset: u32, elem_size: u32, max: u32) -> Result<u32, Error> {
    let count = read_u32_at(buf, offset)?;
    if count > max {
        return Err(Error::LengthExceedsMax);
    }
    let data_len = count
        .checked_mul(elem_size)
        .ok_or(Error::LengthExceedsMax)?;
    let total = 4u32
        .checked_add(data_len)
        .ok_or(Error::LengthExceedsMax)?;
    check_bounds(buf, offset, total)?;
    Ok(total)
}

/// Skip past an XDR variable-length array of variable-size elements.
/// Returns total bytes consumed.
#[inline]
pub fn skip_vec_var(
    buf: &[u8],
    offset: u32,
    max: u32,
    skip_element: impl Fn(&[u8], u32) -> Result<u32, Error>,
) -> Result<u32, Error> {
    let count = read_u32_at(buf, offset)?;
    if count > max {
        return Err(Error::LengthExceedsMax);
    }
    let mut pos = offset.checked_add(4).ok_or(Error::LengthExceedsMax)?;
    for _ in 0..count {
        let elem_len = skip_element(buf, pos)?;
        pos = pos.checked_add(elem_len).ok_or(Error::LengthExceedsMax)?;
    }
    pos.checked_sub(offset).ok_or(Error::Invalid)
}

// ---------------------------------------------------------------------------
// Append support
// ---------------------------------------------------------------------------

/// Append a value to the buffer by serializing it, then return an `XdrRef`
/// pointing to the appended bytes.
///
/// Requires `std` for `WriteXdr` and `alloc` for `Vec`.
#[cfg(feature = "std")]
pub fn append_to_buf<T: WriteXdr>(
    buf: &mut Vec<u8>,
    value: &T,
    limits: Limits,
) -> Result<XdrRef<T>, Error> {
    let offset: u32 = buf
        .len()
        .try_into()
        .map_err(|_| Error::LengthExceedsMax)?;
    let mut cursor = Limited::new(std::io::Cursor::new(Vec::new()), limits);
    value.write_xdr(&mut cursor)?;
    let written = cursor.inner.into_inner();
    let len: u32 = written
        .len()
        .try_into()
        .map_err(|_| Error::LengthExceedsMax)?;
    buf.extend_from_slice(&written);
    Ok(XdrRef::new(offset, len))
}

// ---------------------------------------------------------------------------
// Comparison helpers for generated code
// ---------------------------------------------------------------------------

/// Compare i32 values at offsets in two buffers.
#[inline]
pub fn cmp_i32(buf: &[u8], off: u32, other_buf: &[u8], other_off: u32) -> Result<Ordering, Error> {
    Ok(read_i32_at(buf, off)?.cmp(&read_i32_at(other_buf, other_off)?))
}

/// Compare u32 values at offsets in two buffers.
#[inline]
pub fn cmp_u32(buf: &[u8], off: u32, other_buf: &[u8], other_off: u32) -> Result<Ordering, Error> {
    Ok(read_u32_at(buf, off)?.cmp(&read_u32_at(other_buf, other_off)?))
}

/// Compare i64 values at offsets in two buffers.
#[inline]
pub fn cmp_i64(buf: &[u8], off: u32, other_buf: &[u8], other_off: u32) -> Result<Ordering, Error> {
    Ok(read_i64_at(buf, off)?.cmp(&read_i64_at(other_buf, other_off)?))
}

/// Compare u64 values at offsets in two buffers.
#[inline]
pub fn cmp_u64(buf: &[u8], off: u32, other_buf: &[u8], other_off: u32) -> Result<Ordering, Error> {
    Ok(read_u64_at(buf, off)?.cmp(&read_u64_at(other_buf, other_off)?))
}

/// Compare bool values at offsets in two buffers.
#[inline]
pub fn cmp_bool(
    buf: &[u8],
    off: u32,
    other_buf: &[u8],
    other_off: u32,
) -> Result<Ordering, Error> {
    Ok(read_bool_at(buf, off)?.cmp(&read_bool_at(other_buf, other_off)?))
}

/// Lexicographic comparison of byte slices in two buffers.
#[inline]
pub fn cmp_bytes(
    buf: &[u8],
    off: u32,
    len: u32,
    other_buf: &[u8],
    other_off: u32,
    other_len: u32,
) -> Result<Ordering, Error> {
    let a = read_bytes_at(buf, off, len)?;
    let b = read_bytes_at(other_buf, other_off, other_len)?;
    Ok(a.cmp(b))
}
