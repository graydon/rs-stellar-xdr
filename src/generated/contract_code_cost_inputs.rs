#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractCodeCostInputs is an XDR Struct defined as:
///
/// ```text
/// struct ContractCodeCostInputs {
///     ExtensionPoint ext;
///     uint32 nInstructions;
///     uint32 nFunctions;
///     uint32 nGlobals;
///     uint32 nTableEntries;
///     uint32 nTypes;
///     uint32 nDataSegments;
///     uint32 nElemSegments;
///     uint32 nImports;
///     uint32 nExports;
///     uint32 nDataSegmentBytes;
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
pub struct ContractCodeCostInputs {
    pub ext: ExtensionPoint,
    pub n_instructions: u32,
    pub n_functions: u32,
    pub n_globals: u32,
    pub n_table_entries: u32,
    pub n_types: u32,
    pub n_data_segments: u32,
    pub n_elem_segments: u32,
    pub n_imports: u32,
    pub n_exports: u32,
    pub n_data_segment_bytes: u32,
}

impl ReadXdr for ContractCodeCostInputs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                n_instructions: u32::read_xdr(r)?,
                n_functions: u32::read_xdr(r)?,
                n_globals: u32::read_xdr(r)?,
                n_table_entries: u32::read_xdr(r)?,
                n_types: u32::read_xdr(r)?,
                n_data_segments: u32::read_xdr(r)?,
                n_elem_segments: u32::read_xdr(r)?,
                n_imports: u32::read_xdr(r)?,
                n_exports: u32::read_xdr(r)?,
                n_data_segment_bytes: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeCostInputs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.n_instructions.write_xdr(w)?;
            self.n_functions.write_xdr(w)?;
            self.n_globals.write_xdr(w)?;
            self.n_table_entries.write_xdr(w)?;
            self.n_types.write_xdr(w)?;
            self.n_data_segments.write_xdr(w)?;
            self.n_elem_segments.write_xdr(w)?;
            self.n_imports.write_xdr(w)?;
            self.n_exports.write_xdr(w)?;
            self.n_data_segment_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractCodeCostInputs`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractCodeCostInputs(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractCodeCostInputs {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractCodeCostInputs {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.n_instructions().cmp(&other.n_instructions()))
            .then_with(|| self.n_functions().cmp(&other.n_functions()))
            .then_with(|| self.n_globals().cmp(&other.n_globals()))
            .then_with(|| self.n_table_entries().cmp(&other.n_table_entries()))
            .then_with(|| self.n_types().cmp(&other.n_types()))
            .then_with(|| self.n_data_segments().cmp(&other.n_data_segments()))
            .then_with(|| self.n_elem_segments().cmp(&other.n_elem_segments()))
            .then_with(|| self.n_imports().cmp(&other.n_imports()))
            .then_with(|| self.n_exports().cmp(&other.n_exports()))
            .then_with(|| {
                self.n_data_segment_bytes()
                    .cmp(&other.n_data_segment_bytes())
            })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractCodeCostInputs {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(40).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 40;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyContractCodeCostInputs {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractCodeCostInputs {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractCodeCostInputs {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractCodeCostInputs {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `n_instructions`.
    #[must_use]
    pub fn n_instructions(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_functions`.
    #[must_use]
    pub fn n_functions(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_globals`.
    #[must_use]
    pub fn n_globals(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_table_entries`.
    #[must_use]
    pub fn n_table_entries(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_types`.
    #[must_use]
    pub fn n_types(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 16;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_data_segments`.
    #[must_use]
    pub fn n_data_segments(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 20;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_elem_segments`.
    #[must_use]
    pub fn n_elem_segments(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 24;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_imports`.
    #[must_use]
    pub fn n_imports(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 28;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_exports`.
    #[must_use]
    pub fn n_exports(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 32;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `n_data_segment_bytes`.
    #[must_use]
    pub fn n_data_segment_bytes(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 36;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractCodeCostInputs> for LazyContractCodeCostInputs {
    type Error = Error;
    fn try_from(val: &ContractCodeCostInputs) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractCodeCostInputs> for ContractCodeCostInputs {
    type Error = Error;
    fn try_from(lazy: &LazyContractCodeCostInputs) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
