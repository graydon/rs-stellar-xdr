#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// HashIdPreimage is an XDR Union defined as:
///
/// ```text
/// union HashIDPreimage switch (EnvelopeType type)
/// {
/// case ENVELOPE_TYPE_OP_ID:
///     struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///     } operationID;
/// case ENVELOPE_TYPE_POOL_REVOKE_OP_ID:
///     struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///         PoolID liquidityPoolID;
///         Asset asset;
///     } revokeID;
/// case ENVELOPE_TYPE_CONTRACT_ID:
///     struct
///     {
///         Hash networkID;
///         ContractIDPreimage contractIDPreimage;
///     } contractID;
/// case ENVELOPE_TYPE_SOROBAN_AUTHORIZATION:
///     struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
///         SorobanAuthorizedInvocation invocation;
///     } sorobanAuthorization;
/// #ifdef CAP_0071
/// case ENVELOPE_TYPE_SOROBAN_AUTHORIZATION_WITH_ADDRESS:
///     struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
///         SCAddress address;
///         SorobanAuthorizedInvocation invocation;
///     } sorobanAuthorizationWithAddress;
/// #endif
/// };
/// ```
///
// union with discriminant EnvelopeType
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
pub enum HashIdPreimage {
    OpId(HashIdPreimageOperationId),
    PoolRevokeOpId(HashIdPreimageRevokeId),
    ContractId(HashIdPreimageContractId),
    SorobanAuthorization(HashIdPreimageSorobanAuthorization),
    #[cfg(feature = "cap_0071")]
    SorobanAuthorizationWithAddress(HashIdPreimageSorobanAuthorizationWithAddress),
}

#[cfg(feature = "alloc")]
impl Default for HashIdPreimage {
    fn default() -> Self {
        Self::OpId(HashIdPreimageOperationId::default())
    }
}

impl HashIdPreimage {
    const _VARIANTS: &[EnvelopeType] = &[
        EnvelopeType::OpId,
        EnvelopeType::PoolRevokeOpId,
        EnvelopeType::ContractId,
        EnvelopeType::SorobanAuthorization,
        #[cfg(feature = "cap_0071")]
        EnvelopeType::SorobanAuthorizationWithAddress,
    ];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "OpId",
        "PoolRevokeOpId",
        "ContractId",
        "SorobanAuthorization",
        #[cfg(feature = "cap_0071")]
        "SorobanAuthorizationWithAddress",
    ];
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
            Self::OpId(_) => "OpId",
            Self::PoolRevokeOpId(_) => "PoolRevokeOpId",
            Self::ContractId(_) => "ContractId",
            Self::SorobanAuthorization(_) => "SorobanAuthorization",
            #[cfg(feature = "cap_0071")]
            Self::SorobanAuthorizationWithAddress(_) => "SorobanAuthorizationWithAddress",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpId(_) => EnvelopeType::OpId,
            Self::PoolRevokeOpId(_) => EnvelopeType::PoolRevokeOpId,
            Self::ContractId(_) => EnvelopeType::ContractId,
            Self::SorobanAuthorization(_) => EnvelopeType::SorobanAuthorization,
            #[cfg(feature = "cap_0071")]
            Self::SorobanAuthorizationWithAddress(_) => {
                EnvelopeType::SorobanAuthorizationWithAddress
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HashIdPreimage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for HashIdPreimage {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for HashIdPreimage {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for HashIdPreimage {}

impl ReadXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::OpId => Self::OpId(HashIdPreimageOperationId::read_xdr(r)?),
                EnvelopeType::PoolRevokeOpId => {
                    Self::PoolRevokeOpId(HashIdPreimageRevokeId::read_xdr(r)?)
                }
                EnvelopeType::ContractId => {
                    Self::ContractId(HashIdPreimageContractId::read_xdr(r)?)
                }
                EnvelopeType::SorobanAuthorization => {
                    Self::SorobanAuthorization(HashIdPreimageSorobanAuthorization::read_xdr(r)?)
                }
                #[cfg(feature = "cap_0071")]
                EnvelopeType::SorobanAuthorizationWithAddress => {
                    Self::SorobanAuthorizationWithAddress(
                        HashIdPreimageSorobanAuthorizationWithAddress::read_xdr(r)?,
                    )
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::OpId(v) => v.write_xdr(w)?,
                Self::PoolRevokeOpId(v) => v.write_xdr(w)?,
                Self::ContractId(v) => v.write_xdr(w)?,
                Self::SorobanAuthorization(v) => v.write_xdr(w)?,
                #[cfg(feature = "cap_0071")]
                Self::SorobanAuthorizationWithAddress(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`HashIdPreimage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHashIdPreimage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHashIdPreimage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHashIdPreimage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            6 => self.as_op_id().cmp(&other.as_op_id()),
            7 => self
                .as_pool_revoke_op_id()
                .cmp(&other.as_pool_revoke_op_id()),
            8 => self.as_contract_id().cmp(&other.as_contract_id()),
            9 => self
                .as_soroban_authorization()
                .cmp(&other.as_soroban_authorization()),
            #[cfg(feature = "cap_0071")]
            10 => self
                .as_soroban_authorization_with_address()
                .cmp(&other.as_soroban_authorization_with_address()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHashIdPreimage {
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
            6 => {
                let field_len = <LazyHashIdPreimageOperationId as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len = <LazyHashIdPreimageRevokeId as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len = <LazyHashIdPreimageContractId as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len = <LazyHashIdPreimageSorobanAuthorization as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            #[cfg(feature = "cap_0071")]
            10 => {
                let field_len =
                    <LazyHashIdPreimageSorobanAuthorizationWithAddress as LazyXdr>::xdr_validate(
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
            6 => {
                pos += <LazyHashIdPreimageOperationId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyHashIdPreimageRevokeId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyHashIdPreimageContractId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <LazyHashIdPreimageSorobanAuthorization as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            #[cfg(feature = "cap_0071")]
            10 => {
                pos += <LazyHashIdPreimageSorobanAuthorizationWithAddress as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
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
impl From<LazyHandle> for LazyHashIdPreimage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHashIdPreimage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHashIdPreimage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyHashIdPreimage {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        // Validated — unwrap is safe.
        EnvelopeType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `OpId`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_op_id(&self) -> Option<LazyHashIdPreimageOperationId> {
        if self.discriminant_i32() == 6 {
            Some(<LazyHashIdPreimageOperationId as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `PoolRevokeOpId`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_pool_revoke_op_id(&self) -> Option<LazyHashIdPreimageRevokeId> {
        if self.discriminant_i32() == 7 {
            Some(<LazyHashIdPreimageRevokeId as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ContractId`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_id(&self) -> Option<LazyHashIdPreimageContractId> {
        if self.discriminant_i32() == 8 {
            Some(<LazyHashIdPreimageContractId as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `SorobanAuthorization`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_soroban_authorization(&self) -> Option<LazyHashIdPreimageSorobanAuthorization> {
        if self.discriminant_i32() == 9 {
            Some(<LazyHashIdPreimageSorobanAuthorization as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    #[cfg(feature = "cap_0071")]
    /// Access arm `SorobanAuthorizationWithAddress`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_soroban_authorization_with_address(
        &self,
    ) -> Option<LazyHashIdPreimageSorobanAuthorizationWithAddress> {
        if self.discriminant_i32() == 10 {
            Some(
                <LazyHashIdPreimageSorobanAuthorizationWithAddress as LazyXdr>::from_xdr_at(
                    &self.0, 4,
                ),
            )
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&HashIdPreimage> for LazyHashIdPreimage {
    type Error = Error;
    fn try_from(val: &HashIdPreimage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHashIdPreimage> for HashIdPreimage {
    type Error = Error;
    fn try_from(lazy: &LazyHashIdPreimage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
