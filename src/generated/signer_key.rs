#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SignerKey is an XDR Union defined as:
///
/// ```text
/// union SignerKey switch (SignerKeyType type)
/// {
/// case SIGNER_KEY_TYPE_ED25519:
///     uint256 ed25519;
/// case SIGNER_KEY_TYPE_PRE_AUTH_TX:
///     /* SHA-256 Hash of TransactionSignaturePayload structure */
///     uint256 preAuthTx;
/// case SIGNER_KEY_TYPE_HASH_X:
///     /* Hash of random 256 bit preimage X */
///     uint256 hashX;
/// case SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD:
///     struct
///     {
///         /* Public key that must sign the payload. */
///         uint256 ed25519;
///         /* Payload to be raw signed by ed25519. */
///         opaque payload<64>;
///     } ed25519SignedPayload;
/// };
/// ```
///
// union with discriminant SignerKeyType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayload),
}

#[cfg(feature = "alloc")]
impl Default for SignerKey {
    fn default() -> Self {
        Self::Ed25519(Uint256::default())
    }
}

impl SignerKey {
    const _VARIANTS: &[SignerKeyType] = &[
        SignerKeyType::Ed25519,
        SignerKeyType::PreAuthTx,
        SignerKeyType::HashX,
        SignerKeyType::Ed25519SignedPayload,
    ];
    pub const VARIANTS: [SignerKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Ed25519", "PreAuthTx", "HashX", "Ed25519SignedPayload"];
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
            Self::PreAuthTx(_) => "PreAuthTx",
            Self::HashX(_) => "HashX",
            Self::Ed25519SignedPayload(_) => "Ed25519SignedPayload",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SignerKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => SignerKeyType::Ed25519,
            Self::PreAuthTx(_) => SignerKeyType::PreAuthTx,
            Self::HashX(_) => SignerKeyType::HashX,
            Self::Ed25519SignedPayload(_) => SignerKeyType::Ed25519SignedPayload,
        }
    }

    #[must_use]
    pub const fn variants() -> [SignerKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SignerKey {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SignerKeyType> for SignerKey {
    #[must_use]
    fn discriminant(&self) -> SignerKeyType {
        Self::discriminant(self)
    }
}

impl Variants<SignerKeyType> for SignerKey {
    fn variants() -> slice::Iter<'static, SignerKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SignerKeyType> for SignerKey {}

impl ReadXdr for SignerKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SignerKeyType = <SignerKeyType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SignerKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
                SignerKeyType::PreAuthTx => Self::PreAuthTx(Uint256::read_xdr(r)?),
                SignerKeyType::HashX => Self::HashX(Uint256::read_xdr(r)?),
                SignerKeyType::Ed25519SignedPayload => {
                    Self::Ed25519SignedPayload(SignerKeyEd25519SignedPayload::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SignerKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Ed25519(v) => v.write_xdr(w)?,
                Self::PreAuthTx(v) => v.write_xdr(w)?,
                Self::HashX(v) => v.write_xdr(w)?,
                Self::Ed25519SignedPayload(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SignerKey`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySignerKey(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySignerKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySignerKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_ed25519().cmp(&other.as_ed25519()),
            1 => self.as_pre_auth_tx().cmp(&other.as_pre_auth_tx()),
            2 => self.as_hash_x().cmp(&other.as_hash_x()),
            3 => self
                .as_ed25519_signed_payload()
                .cmp(&other.as_ed25519_signed_payload()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySignerKey {
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
            1 => {
                let field_len =
                    <LazyUint256 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyUint256 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <LazySignerKeyEd25519SignedPayload as LazyXdr>::xdr_validate(
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
            1 => {
                pos += <LazyUint256 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyUint256 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos +=
                    <LazySignerKeyEd25519SignedPayload as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySignerKey {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySignerKey {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySignerKey {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySignerKey {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> SignerKeyType {
        // Validated — unwrap is safe.
        SignerKeyType::try_from(self.discriminant_i32()).unwrap()
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
    /// Access arm `PreAuthTx`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_pre_auth_tx(&self) -> Option<LazyUint256> {
        if self.discriminant_i32() == 1 {
            Some(<LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `HashX`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_hash_x(&self) -> Option<LazyUint256> {
        if self.discriminant_i32() == 2 {
            Some(<LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Ed25519SignedPayload`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ed25519_signed_payload(&self) -> Option<LazySignerKeyEd25519SignedPayload> {
        if self.discriminant_i32() == 3 {
            Some(<LazySignerKeyEd25519SignedPayload as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SignerKey> for LazySignerKey {
    type Error = Error;
    fn try_from(val: &SignerKey) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySignerKey> for SignerKey {
    type Error = Error;
    fn try_from(lazy: &LazySignerKey) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
