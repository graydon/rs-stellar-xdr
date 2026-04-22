#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanAuthorizedFunction is an XDR Union defined as:
///
/// ```text
/// union SorobanAuthorizedFunction switch (SorobanAuthorizedFunctionType type)
/// {
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CONTRACT_FN:
///     InvokeContractArgs contractFn;
/// // This variant of auth payload for creating new contract instances
/// // doesn't allow specifying the constructor arguments, creating contracts
/// // with constructors that take arguments is only possible by authorizing
/// // `SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_V2_HOST_FN`
/// // (protocol 22+).
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_HOST_FN:
///     CreateContractArgs createContractHostFn;
/// // This variant of auth payload for creating new contract instances
/// // is only accepted in and after protocol 22. It allows authorizing the
/// // contract constructor arguments.
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_V2_HOST_FN:
///     CreateContractArgsV2 createContractV2HostFn;
/// };
/// ```
///
// union with discriminant SorobanAuthorizedFunctionType
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
pub enum SorobanAuthorizedFunction {
    ContractFn(InvokeContractArgs),
    CreateContractHostFn(CreateContractArgs),
    CreateContractV2HostFn(CreateContractArgsV2),
}

#[cfg(feature = "alloc")]
impl Default for SorobanAuthorizedFunction {
    fn default() -> Self {
        Self::ContractFn(InvokeContractArgs::default())
    }
}

impl SorobanAuthorizedFunction {
    const _VARIANTS: &[SorobanAuthorizedFunctionType] = &[
        SorobanAuthorizedFunctionType::ContractFn,
        SorobanAuthorizedFunctionType::CreateContractHostFn,
        SorobanAuthorizedFunctionType::CreateContractV2HostFn,
    ];
    pub const VARIANTS: [SorobanAuthorizedFunctionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ContractFn",
        "CreateContractHostFn",
        "CreateContractV2HostFn",
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
            Self::ContractFn(_) => "ContractFn",
            Self::CreateContractHostFn(_) => "CreateContractHostFn",
            Self::CreateContractV2HostFn(_) => "CreateContractV2HostFn",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ContractFn(_) => SorobanAuthorizedFunctionType::ContractFn,
            Self::CreateContractHostFn(_) => SorobanAuthorizedFunctionType::CreateContractHostFn,
            Self::CreateContractV2HostFn(_) => {
                SorobanAuthorizedFunctionType::CreateContractV2HostFn
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [SorobanAuthorizedFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SorobanAuthorizedFunction {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {
    #[must_use]
    fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        Self::discriminant(self)
    }
}

impl Variants<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {
    fn variants() -> slice::Iter<'static, SorobanAuthorizedFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {}

impl ReadXdr for SorobanAuthorizedFunction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SorobanAuthorizedFunctionType =
                <SorobanAuthorizedFunctionType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SorobanAuthorizedFunctionType::ContractFn => {
                    Self::ContractFn(InvokeContractArgs::read_xdr(r)?)
                }
                SorobanAuthorizedFunctionType::CreateContractHostFn => {
                    Self::CreateContractHostFn(CreateContractArgs::read_xdr(r)?)
                }
                SorobanAuthorizedFunctionType::CreateContractV2HostFn => {
                    Self::CreateContractV2HostFn(CreateContractArgsV2::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizedFunction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ContractFn(v) => v.write_xdr(w)?,
                Self::CreateContractHostFn(v) => v.write_xdr(w)?,
                Self::CreateContractV2HostFn(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanAuthorizedFunction`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanAuthorizedFunction(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanAuthorizedFunction {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanAuthorizedFunction {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_contract_fn().cmp(&other.as_contract_fn()),
            1 => self
                .as_create_contract_host_fn()
                .cmp(&other.as_create_contract_host_fn()),
            2 => self
                .as_create_contract_v2_host_fn()
                .cmp(&other.as_create_contract_v2_host_fn()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanAuthorizedFunction {
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
                    <LazyInvokeContractArgs as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyCreateContractArgs as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyCreateContractArgsV2 as LazyXdr>::xdr_validate(
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
                pos += <LazyInvokeContractArgs as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyCreateContractArgs as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyCreateContractArgsV2 as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanAuthorizedFunction {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanAuthorizedFunction {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanAuthorizedFunction {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanAuthorizedFunction {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        // Validated — unwrap is safe.
        SorobanAuthorizedFunctionType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `ContractFn`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract_fn(&self) -> Option<LazyInvokeContractArgs> {
        if self.discriminant_i32() == 0 {
            Some(<LazyInvokeContractArgs as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `CreateContractHostFn`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_create_contract_host_fn(&self) -> Option<LazyCreateContractArgs> {
        if self.discriminant_i32() == 1 {
            Some(<LazyCreateContractArgs as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `CreateContractV2HostFn`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_create_contract_v2_host_fn(&self) -> Option<LazyCreateContractArgsV2> {
        if self.discriminant_i32() == 2 {
            Some(<LazyCreateContractArgsV2 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanAuthorizedFunction> for LazySorobanAuthorizedFunction {
    type Error = Error;
    fn try_from(val: &SorobanAuthorizedFunction) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanAuthorizedFunction> for SorobanAuthorizedFunction {
    type Error = Error;
    fn try_from(lazy: &LazySorobanAuthorizedFunction) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
