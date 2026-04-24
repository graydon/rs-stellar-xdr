#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// StoredTransactionSet is an XDR Union defined as:
///
/// ```text
/// union StoredTransactionSet switch (int v)
/// {
/// case 0:
/// 	TransactionSet txSet;
/// case 1:
/// 	GeneralizedTransactionSet generalizedTxSet;
/// };
/// ```
///
// union with discriminant i32
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum StoredTransactionSet {
    V0(TransactionSet),
    V1(GeneralizedTransactionSet),
}

#[cfg(feature = "alloc")]
impl Default for StoredTransactionSet {
    fn default() -> Self {
        Self::V0(TransactionSet::default())
    }
}

impl StoredTransactionSet {
    const _VARIANTS: &[i32] = &[0, 1];
    pub const VARIANTS: [i32; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0", "V1"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::V0(_) => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
            Self::V1(_) => 1,
        }
    }

    #[must_use]
    pub const fn variants() -> [i32; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for StoredTransactionSet {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<i32> for StoredTransactionSet {
    #[must_use]
    fn discriminant(&self) -> i32 {
        Self::discriminant(self)
    }
}

impl Variants<i32> for StoredTransactionSet {
    fn variants() -> slice::Iter<'static, i32> {
        Self::VARIANTS.iter()
    }
}

impl Union<i32> for StoredTransactionSet {}

impl ReadXdr for StoredTransactionSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                0 => Self::V0(TransactionSet::read_xdr(r)?),
                1 => Self::V1(GeneralizedTransactionSet::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for StoredTransactionSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0(v) => v.write_xdr(w)?,
                Self::V1(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`StoredTransactionSet`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStoredTransactionSet(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyStoredTransactionSet {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyStoredTransactionSet {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_v0().cmp(&other.as_v0()),
            1 => self.as_v1().cmp(&other.as_v1()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyStoredTransactionSet {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyTransactionSet as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len = <LazyGeneralizedTransactionSet as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyStoredTransactionSet {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyStoredTransactionSet {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyStoredTransactionSet {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyStoredTransactionSet {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> i32 {
        <i32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access arm `V0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_v0(&self) -> Option<LazyTransactionSet> {
        if self.discriminant_i32() == 0 {
            Some(<LazyTransactionSet as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `V1`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_v1(&self) -> Option<LazyGeneralizedTransactionSet> {
        if self.discriminant_i32() == 1 {
            Some(<LazyGeneralizedTransactionSet as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&StoredTransactionSet> for LazyStoredTransactionSet {
    type Error = Error;
    fn try_from(val: &StoredTransactionSet) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyStoredTransactionSet> for StoredTransactionSet {
    type Error = Error;
    fn try_from(lazy: &LazyStoredTransactionSet) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
