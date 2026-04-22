#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// MuxedAccount is an XDR Union defined as:
///
/// ```text
/// union MuxedAccount switch (CryptoKeyType type)
/// {
/// case KEY_TYPE_ED25519:
///     uint256 ed25519;
/// case KEY_TYPE_MUXED_ED25519:
///     struct
///     {
///         uint64 id;
///         uint256 ed25519;
///     } med25519;
/// };
/// ```
///
// union with discriminant CryptoKeyType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum MuxedAccount {
    Ed25519(Uint256),
    MuxedEd25519(MuxedAccountMed25519),
}

#[cfg(feature = "alloc")]
impl Default for MuxedAccount {
    fn default() -> Self {
        Self::Ed25519(Uint256::default())
    }
}

impl MuxedAccount {
    const _VARIANTS: &[CryptoKeyType] = &[CryptoKeyType::Ed25519, CryptoKeyType::MuxedEd25519];
    pub const VARIANTS: [CryptoKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Ed25519", "MuxedEd25519"];
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
            Self::Ed25519(_) => "Ed25519",
            Self::MuxedEd25519(_) => "MuxedEd25519",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CryptoKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => CryptoKeyType::Ed25519,
            Self::MuxedEd25519(_) => CryptoKeyType::MuxedEd25519,
        }
    }

    #[must_use]
    pub const fn variants() -> [CryptoKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for MuxedAccount {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CryptoKeyType> for MuxedAccount {
    #[must_use]
    fn discriminant(&self) -> CryptoKeyType {
        Self::discriminant(self)
    }
}

impl Variants<CryptoKeyType> for MuxedAccount {
    fn variants() -> slice::Iter<'static, CryptoKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Union<CryptoKeyType> for MuxedAccount {}

impl ReadXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CryptoKeyType = <CryptoKeyType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CryptoKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
                CryptoKeyType::MuxedEd25519 => {
                    Self::MuxedEd25519(MuxedAccountMed25519::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Ed25519(v) => v.write_xdr(w)?,
                Self::MuxedEd25519(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`MuxedAccount`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyMuxedAccount(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyMuxedAccount {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyMuxedAccount {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_ed25519().cmp(&other.as_ed25519()),
            256 => self.as_muxed_ed25519().cmp(&other.as_muxed_ed25519()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyMuxedAccount {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyUint256 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            256 => {
                let field_len = <LazyMuxedAccountMed25519 as LazyXdr>::xdr_validate(
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
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyUint256 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            256 => {
                pos += <LazyMuxedAccountMed25519 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyMuxedAccount {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyMuxedAccount {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyMuxedAccount {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyMuxedAccount {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> CryptoKeyType {
        // Validated — unwrap is safe.
        CryptoKeyType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Ed25519`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ed25519(&self) -> Option<LazyUint256> {
        if self.discriminant_i32() == 0 {
            Some(<LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `MuxedEd25519`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_muxed_ed25519(&self) -> Option<LazyMuxedAccountMed25519> {
        if self.discriminant_i32() == 256 {
            Some(<LazyMuxedAccountMed25519 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&MuxedAccount> for LazyMuxedAccount {
    type Error = Error;
    fn try_from(val: &MuxedAccount) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyMuxedAccount> for MuxedAccount {
    type Error = Error;
    fn try_from(lazy: &LazyMuxedAccount) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
