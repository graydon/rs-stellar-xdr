#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// AccountEntryExtensionV1 is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntryExtensionV1
/// {
///     Liabilities liabilities;
///
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 2:
///         AccountEntryExtensionV2 v2;
///     }
///     ext;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AccountEntryExtensionV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1Ext,
}

impl ReadXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liabilities: Liabilities::read_xdr(r)?,
                ext: AccountEntryExtensionV1Ext::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liabilities.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`AccountEntryExtensionV1`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyAccountEntryExtensionV1(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyAccountEntryExtensionV1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyAccountEntryExtensionV1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.liabilities().cmp(&other.liabilities()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyAccountEntryExtensionV1 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(16).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyLiabilities as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyAccountEntryExtensionV1Ext as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 16;
        pos += <LazyAccountEntryExtensionV1Ext as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyAccountEntryExtensionV1 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyAccountEntryExtensionV1 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyAccountEntryExtensionV1 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyAccountEntryExtensionV1 {
    /// Access field `liabilities`.
    #[must_use]
    pub fn liabilities(&self) -> LazyLiabilities {
        <LazyLiabilities as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyAccountEntryExtensionV1Ext {
        <LazyAccountEntryExtensionV1Ext as LazyXdr>::from_xdr_at(&self.0, 16)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&AccountEntryExtensionV1> for LazyAccountEntryExtensionV1 {
    type Error = Error;
    fn try_from(val: &AccountEntryExtensionV1) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyAccountEntryExtensionV1> for AccountEntryExtensionV1 {
    type Error = Error;
    fn try_from(lazy: &LazyAccountEntryExtensionV1) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
