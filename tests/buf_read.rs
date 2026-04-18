#![cfg(all(any(feature = "curr", feature = "next"), not(all(feature = "curr", feature = "next"))))]
#![cfg(all(feature = "std", feature = "buf_read"))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::*;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Serialize a value into `buf` and return its `XdrRef`.
fn push<T: WriteXdr>(buf: &mut Vec<u8>, value: &T) -> XdrRef<T> {
    append_to_buf(buf, value, Limits::none()).unwrap()
}

/// Validate and return an `XdrRef<T>` at offset 0 of a single-value buffer.
fn ref_at_zero<T: WriteXdr + ReadXdr>(value: &T) -> (Vec<u8>, XdrRef<T>)
where
    T: XdrRefValidate,
{
    let mut buf = Vec::new();
    let r = push(&mut buf, value);
    // Re-validate from scratch to exercise the validate path.
    let r2 = T::xdr_ref_from(&buf, 0, &mut RefLimits::none()).unwrap();
    assert_eq!(r.offset(), r2.offset());
    assert_eq!(r.len(), r2.len());
    (buf, r)
}

/// Trait to call `xdr_ref_from` generically. Each generated type already has
/// this as an inherent method, but we need a trait to be generic.
trait XdrRefValidate: Sized {
    fn xdr_ref_from(
        buf: &[u8],
        offset: u32,
        limits: &mut RefLimits,
    ) -> Result<XdrRef<Self>, Error>;
}

macro_rules! impl_xdr_ref_validate {
    ($($ty:ty),* $(,)?) => {
        $(
            impl XdrRefValidate for $ty {
                fn xdr_ref_from(
                    buf: &[u8],
                    offset: u32,
                    limits: &mut RefLimits,
                ) -> Result<XdrRef<Self>, Error> {
                    <$ty>::xdr_ref_from(buf, offset, limits)
                }
            }
        )*
    };
}

impl_xdr_ref_validate!(
    ScVal,
    ScMapEntry,
    ScVec,
    ScMap,
    UInt128Parts,
    Int128Parts,
    ScError,
    ScNonceKey,
    ScContractInstance,
    ScAddress,
);

// ===========================================================================
// Primitive ScVal variants
// ===========================================================================

#[test]
fn scval_void() {
    let val = ScVal::Void;
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Void);
    assert_eq!(r.materialize(&buf).unwrap(), val);
}

#[test]
fn scval_bool_true() {
    let val = ScVal::Bool(true);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Bool);
    let inner = r.as_bool(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), true);
    assert_eq!(r.materialize(&buf).unwrap(), val);
}

#[test]
fn scval_bool_false() {
    let val = ScVal::Bool(false);
    let (buf, r) = ref_at_zero(&val);
    let inner = r.as_bool(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), false);
}

#[test]
fn scval_u32() {
    let val = ScVal::U32(0xDEAD_BEEF);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::U32);
    let inner = r.as_u32(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), 0xDEAD_BEEF);
    // Wrong-arm accessor returns None.
    assert!(r.as_i32(&buf).unwrap().is_none());
    assert!(r.as_u64(&buf).unwrap().is_none());
}

#[test]
fn scval_i32() {
    let val = ScVal::I32(-42);
    let (buf, r) = ref_at_zero(&val);
    let inner = r.as_i32(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), -42);
}

#[test]
fn scval_u64() {
    let val = ScVal::U64(u64::MAX);
    let (buf, r) = ref_at_zero(&val);
    let inner = r.as_u64(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), u64::MAX);
}

#[test]
fn scval_i64() {
    let val = ScVal::I64(i64::MIN);
    let (buf, r) = ref_at_zero(&val);
    let inner = r.as_i64(&buf).unwrap().unwrap();
    assert_eq!(inner.get(&buf).unwrap(), i64::MIN);
}

// ===========================================================================
// 128-bit and 256-bit parts
// ===========================================================================

#[test]
fn scval_u128_field_access() {
    let parts = UInt128Parts {
        hi: 0x0102_0304_0506_0708,
        lo: 0x090A_0B0C_0D0E_0F10,
    };
    let val = ScVal::U128(parts.clone());
    let (buf, r) = ref_at_zero(&val);
    let u128_ref = r.as_u128(&buf).unwrap().unwrap();
    assert_eq!(u128_ref.hi(&buf).unwrap().get(&buf).unwrap(), parts.hi);
    assert_eq!(u128_ref.lo(&buf).unwrap().get(&buf).unwrap(), parts.lo);
    assert_eq!(r.materialize(&buf).unwrap(), val);
}

#[test]
fn scval_i128_field_access() {
    let parts = Int128Parts { hi: -1, lo: 42 };
    let val = ScVal::I128(parts.clone());
    let (buf, r) = ref_at_zero(&val);
    let i128_ref = r.as_i128(&buf).unwrap().unwrap();
    assert_eq!(i128_ref.hi(&buf).unwrap().get(&buf).unwrap(), -1i64);
    assert_eq!(i128_ref.lo(&buf).unwrap().get(&buf).unwrap(), 42u64);
}

// ===========================================================================
// Timepoint and Duration
// ===========================================================================

#[test]
fn scval_timepoint() {
    let val = ScVal::Timepoint(TimePoint(1_700_000_000));
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Timepoint);
    let tp_ref = r.as_timepoint(&buf).unwrap().unwrap();
    assert_eq!(tp_ref.materialize(&buf).unwrap(), TimePoint(1_700_000_000));
}

#[test]
fn scval_duration() {
    let val = ScVal::Duration(Duration(86400));
    let (buf, r) = ref_at_zero(&val);
    let dur_ref = r.as_duration(&buf).unwrap().unwrap();
    assert_eq!(dur_ref.materialize(&buf).unwrap(), Duration(86400));
}

// ===========================================================================
// Variable-length opaque: ScBytes, ScString, ScSymbol
// ===========================================================================

#[test]
fn scval_bytes() {
    let data: Vec<u8> = (0..17).collect(); // 17 bytes → padded to 20
    let val = ScVal::Bytes(ScBytes(data.clone().try_into().unwrap()));
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Bytes);
    let bytes_ref = r.as_bytes(&buf).unwrap().unwrap();
    let mat = bytes_ref.materialize(&buf).unwrap();
    assert_eq!(AsRef::<[u8]>::as_ref(&mat), data.as_slice());
}

#[test]
fn scval_string() {
    let val = ScVal::String(ScString(StringM::try_from("hello world").unwrap()));
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::String);
    let str_ref = r.as_string(&buf).unwrap().unwrap();
    let mat = str_ref.materialize(&buf).unwrap();
    assert_eq!(mat.to_string(), "hello world");
}

#[test]
fn scval_symbol() {
    let val = ScVal::Symbol(ScSymbol(StringM::try_from("transfer").unwrap()));
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Symbol);
    let sym_ref = r.as_symbol(&buf).unwrap().unwrap();
    let mat = sym_ref.materialize(&buf).unwrap();
    assert_eq!(mat.to_string(), "transfer");
}

#[test]
fn scval_empty_bytes() {
    let val = ScVal::Bytes(ScBytes(BytesM::default()));
    let (buf, r) = ref_at_zero(&val);
    let bytes_ref = r.as_bytes(&buf).unwrap().unwrap();
    let mat = bytes_ref.materialize(&buf).unwrap();
    assert!(AsRef::<[u8]>::as_ref(&mat).is_empty());
}

// ===========================================================================
// ScVal::Vec (Option<ScVec>)
// ===========================================================================

#[test]
fn scval_vec_none() {
    let val = ScVal::Vec(None);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Vec);
    let opt_ref = r.as_vec(&buf).unwrap().unwrap();
    assert!(opt_ref.inner(&buf).unwrap().is_none());
    assert_eq!(r.materialize(&buf).unwrap(), val);
}

#[test]
fn scval_vec_empty() {
    let val = ScVal::Vec(Some(ScVec(VecM::default())));
    let (buf, r) = ref_at_zero(&val);
    let opt_ref = r.as_vec(&buf).unwrap().unwrap();
    assert!(opt_ref.inner(&buf).unwrap().is_some());
    let vec_ref = opt_ref.inner(&buf).unwrap().unwrap();
    let mat = vec_ref.materialize(&buf).unwrap();
    assert!(mat.0.is_empty());
}

#[test]
fn scval_vec_with_elements() {
    let elements: VecM<ScVal> = vec![
        ScVal::I32(10),
        ScVal::I32(20),
        ScVal::Bool(true),
    ]
    .try_into()
    .unwrap();
    let val = ScVal::Vec(Some(ScVec(elements)));
    let (buf, r) = ref_at_zero(&val);
    let vec_ref = r
        .as_vec(&buf)
        .unwrap()
        .unwrap()
        .inner(&buf)
        .unwrap()
        .unwrap();
    let mat = vec_ref.materialize(&buf).unwrap();
    assert_eq!(mat.0.len(), 3);
    assert_eq!(mat.0[0], ScVal::I32(10));
    assert_eq!(mat.0[1], ScVal::I32(20));
    assert_eq!(mat.0[2], ScVal::Bool(true));
}

// ===========================================================================
// ScVal::Map (Option<ScMap>)
// ===========================================================================

#[test]
fn scval_map_none() {
    let val = ScVal::Map(None);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Map);
    let opt_ref = r.as_map(&buf).unwrap().unwrap();
    assert!(opt_ref.inner(&buf).unwrap().is_none());
}

#[test]
fn scval_map_with_entries() {
    let entries: VecM<ScMapEntry> = vec![
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::try_from("a").unwrap())),
            val: ScVal::I32(1),
        },
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::try_from("bb").unwrap())),
            val: ScVal::U64(999),
        },
    ]
    .try_into()
    .unwrap();
    let val = ScVal::Map(Some(ScMap(entries)));
    let (buf, r) = ref_at_zero(&val);

    let map_ref = r
        .as_map(&buf)
        .unwrap()
        .unwrap()
        .inner(&buf)
        .unwrap()
        .unwrap();
    let mat_map = map_ref.materialize(&buf).unwrap();
    assert_eq!(mat_map.0.len(), 2);
    assert_eq!(
        mat_map.0[0].key,
        ScVal::Symbol(ScSymbol(StringM::try_from("a").unwrap()))
    );
    assert_eq!(mat_map.0[0].val, ScVal::I32(1));
    assert_eq!(mat_map.0[1].val, ScVal::U64(999));
}

// ===========================================================================
// ScMapEntry — direct field access via refs
// ===========================================================================

#[test]
fn scmap_entry_field_access() {
    let entry = ScMapEntry {
        key: ScVal::U32(42),
        val: ScVal::I64(-100),
    };
    let (buf, r) = ref_at_zero(&entry);

    // Navigate to .key
    let key_ref = r.key(&buf).unwrap();
    assert_eq!(key_ref.discriminant(&buf).unwrap(), ScValType::U32);
    assert_eq!(key_ref.as_u32(&buf).unwrap().unwrap().get(&buf).unwrap(), 42);

    // Navigate to .val (scans past key)
    let val_ref = r.val(&buf).unwrap();
    assert_eq!(val_ref.discriminant(&buf).unwrap(), ScValType::I64);
    assert_eq!(
        val_ref.as_i64(&buf).unwrap().unwrap().get(&buf).unwrap(),
        -100
    );
}

#[test]
fn scmap_entry_variable_size_key() {
    // Key is variable-length, so .val() must scan past it.
    let entry = ScMapEntry {
        key: ScVal::Bytes(ScBytes(vec![1, 2, 3, 4, 5].try_into().unwrap())),
        val: ScVal::U32(77),
    };
    let (buf, r) = ref_at_zero(&entry);

    let key_ref = r.key(&buf).unwrap();
    assert_eq!(key_ref.discriminant(&buf).unwrap(), ScValType::Bytes);

    let val_ref = r.val(&buf).unwrap();
    assert_eq!(
        val_ref.as_u32(&buf).unwrap().unwrap().get(&buf).unwrap(),
        77
    );
}

// ===========================================================================
// ScError
// ===========================================================================

#[test]
fn scval_error() {
    let val = ScVal::Error(ScError::Budget(ScErrorCode::ExceededLimit));
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Error);
    let err_ref = r.as_error(&buf).unwrap().unwrap();
    assert_eq!(err_ref.discriminant(&buf).unwrap(), ScErrorType::Budget);
    assert_eq!(r.materialize(&buf).unwrap(), val);
}

#[test]
fn scerror_contract() {
    let err = ScError::Contract(12345);
    let (buf, r) = ref_at_zero(&err);
    assert_eq!(r.discriminant(&buf).unwrap(), ScErrorType::Contract);
    let u32_ref = r.as_contract(&buf).unwrap().unwrap();
    assert_eq!(u32_ref.get(&buf).unwrap(), 12345);
    // Other arms return None.
    assert!(r.as_wasm_vm(&buf).unwrap().is_none());
}

// ===========================================================================
// ScNonceKey
// ===========================================================================

#[test]
fn scval_ledger_key_nonce() {
    let nonce_key = ScNonceKey { nonce: -9999 };
    let val = ScVal::LedgerKeyNonce(nonce_key.clone());
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::LedgerKeyNonce);
    let nk_ref = r.as_ledger_key_nonce(&buf).unwrap().unwrap();
    let nonce_ref = nk_ref.nonce(&buf).unwrap();
    assert_eq!(nonce_ref.get(&buf).unwrap(), -9999i64);
}

// ===========================================================================
// ScVal::LedgerKeyContractInstance (void arm)
// ===========================================================================

#[test]
fn scval_ledger_key_contract_instance() {
    let val = ScVal::LedgerKeyContractInstance;
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(
        r.discriminant(&buf).unwrap(),
        ScValType::LedgerKeyContractInstance
    );
    assert_eq!(r.materialize(&buf).unwrap(), val);
    // All typed accessors return None.
    assert!(r.as_bool(&buf).unwrap().is_none());
    assert!(r.as_u32(&buf).unwrap().is_none());
}

// ===========================================================================
// Multiple values in the same buffer
// ===========================================================================

#[test]
fn multiple_values_in_buffer() {
    let mut buf = Vec::new();
    let r1 = push(&mut buf, &ScVal::I32(100));
    let r2 = push(&mut buf, &ScVal::U64(200));
    let r3 = push(&mut buf, &ScVal::Bool(false));

    assert_eq!(
        r1.as_i32(&buf).unwrap().unwrap().get(&buf).unwrap(),
        100
    );
    assert_eq!(
        r2.as_u64(&buf).unwrap().unwrap().get(&buf).unwrap(),
        200
    );
    assert_eq!(
        r3.as_bool(&buf).unwrap().unwrap().get(&buf).unwrap(),
        false
    );

    // Offsets are sequential.
    assert_eq!(r1.offset(), 0);
    assert!(r2.offset() > r1.offset());
    assert!(r3.offset() > r2.offset());
}

#[test]
fn mixed_types_in_buffer() {
    let mut buf = Vec::new();
    let entry = ScMapEntry {
        key: ScVal::Symbol(ScSymbol(StringM::try_from("x").unwrap())),
        val: ScVal::I32(7),
    };
    let r_entry = push(&mut buf, &entry);
    let r_u128 = push(
        &mut buf,
        &UInt128Parts {
            hi: 0xAAAA,
            lo: 0xBBBB,
        },
    );
    let r_val = push(&mut buf, &ScVal::Void);

    // Access each via its own ref.
    assert_eq!(
        r_entry
            .key(&buf)
            .unwrap()
            .as_symbol(&buf)
            .unwrap()
            .unwrap()
            .materialize(&buf)
            .unwrap()
            .to_string(),
        "x"
    );
    assert_eq!(r_u128.hi(&buf).unwrap().get(&buf).unwrap(), 0xAAAA);
    assert_eq!(r_u128.lo(&buf).unwrap().get(&buf).unwrap(), 0xBBBB);
    assert_eq!(r_val.discriminant(&buf).unwrap(), ScValType::Void);
}

// ===========================================================================
// Materialization roundtrip
// ===========================================================================

#[test]
fn materialize_roundtrip_complex() {
    let val = ScVal::Map(Some(ScMap(
        vec![
            ScMapEntry {
                key: ScVal::Symbol(ScSymbol(StringM::try_from("amount").unwrap())),
                val: ScVal::I128(Int128Parts { hi: 0, lo: 1_000_000 }),
            },
            ScMapEntry {
                key: ScVal::Symbol(ScSymbol(StringM::try_from("from").unwrap())),
                val: ScVal::Bytes(ScBytes(vec![0xAB; 32].try_into().unwrap())),
            },
        ]
        .try_into()
        .unwrap(),
    )));
    let (buf, r) = ref_at_zero(&val);
    let roundtrip = r.materialize(&buf).unwrap();
    assert_eq!(roundtrip, val);
}

// ===========================================================================
// Comparison (cmp_in)
// ===========================================================================

#[test]
fn cmp_in_equal() {
    let val = ScVal::I32(42);
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &val);
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &val);
    assert_eq!(
        r1.cmp_in(&buf1, &r2, &buf2).unwrap(),
        Ordering::Equal
    );
}

#[test]
fn cmp_in_different_discriminant() {
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &ScVal::I32(0));
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &ScVal::U64(0));
    // Different discriminants → ordered by discriminant value.
    let ord = r1.cmp_in(&buf1, &r2, &buf2).unwrap();
    assert_ne!(ord, Ordering::Equal);
}

#[test]
fn cmp_in_same_discriminant_different_value() {
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &ScVal::U32(10));
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &ScVal::U32(20));
    assert_eq!(
        r1.cmp_in(&buf1, &r2, &buf2).unwrap(),
        Ordering::Less
    );
    assert_eq!(
        r2.cmp_in(&buf2, &r1, &buf1).unwrap(),
        Ordering::Greater
    );
}

#[test]
fn cmp_in_struct_field_by_field() {
    let e1 = ScMapEntry {
        key: ScVal::I32(1),
        val: ScVal::I32(100),
    };
    let e2 = ScMapEntry {
        key: ScVal::I32(1),
        val: ScVal::I32(200),
    };
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &e1);
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &e2);
    // Same key, different val → ordered by val.
    assert_eq!(
        r1.cmp_in(&buf1, &r2, &buf2).unwrap(),
        Ordering::Less
    );
}

#[test]
fn cmp_in_same_buffer() {
    let mut buf = Vec::new();
    let r1 = push(&mut buf, &ScVal::U32(5));
    let r2 = push(&mut buf, &ScVal::U32(10));
    assert_eq!(
        r1.cmp_in(&buf, &r2, &buf).unwrap(),
        Ordering::Less
    );
}

// ===========================================================================
// eq_in
// ===========================================================================

#[test]
fn eq_in_same_value() {
    let val = ScVal::Symbol(ScSymbol(StringM::try_from("test").unwrap()));
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &val);
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &val);
    assert!(r1.eq_in(&buf1, &r2, &buf2));
}

#[test]
fn eq_in_different_value() {
    let mut buf1 = Vec::new();
    let r1 = push(&mut buf1, &ScVal::I32(1));
    let mut buf2 = Vec::new();
    let r2 = push(&mut buf2, &ScVal::I32(2));
    assert!(!r1.eq_in(&buf1, &r2, &buf2));
}

// ===========================================================================
// Nested structure navigation
// ===========================================================================

#[test]
fn nested_map_entry_navigation() {
    // A map where the value of an entry is itself a vec of values.
    let inner_vec = ScVal::Vec(Some(ScVec(
        vec![ScVal::U32(1), ScVal::U32(2), ScVal::U32(3)]
            .try_into()
            .unwrap(),
    )));
    let entry = ScMapEntry {
        key: ScVal::Symbol(ScSymbol(StringM::try_from("items").unwrap())),
        val: inner_vec.clone(),
    };
    let (buf, r) = ref_at_zero(&entry);

    // Navigate: entry -> val -> vec -> materialize
    let val_ref = r.val(&buf).unwrap();
    assert_eq!(val_ref.discriminant(&buf).unwrap(), ScValType::Vec);
    let vec_opt_ref = val_ref.as_vec(&buf).unwrap().unwrap();
    let vec_ref = vec_opt_ref.inner(&buf).unwrap().unwrap();
    let materialized = vec_ref.materialize(&buf).unwrap();
    assert_eq!(materialized.0.len(), 3);
    assert_eq!(materialized.0[2], ScVal::U32(3));
}

#[test]
fn deeply_nested_scval() {
    // ScVal::Map containing entries where values are also maps.
    let inner_map = ScVal::Map(Some(ScMap(
        vec![ScMapEntry {
            key: ScVal::I32(99),
            val: ScVal::Bool(true),
        }]
        .try_into()
        .unwrap(),
    )));
    let outer = ScVal::Map(Some(ScMap(
        vec![ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::try_from("nested").unwrap())),
            val: inner_map,
        }]
        .try_into()
        .unwrap(),
    )));
    let (buf, r) = ref_at_zero(&outer);

    // Materialize the whole thing and verify.
    let mat = r.materialize(&buf).unwrap();
    assert_eq!(mat, outer);
}

// ===========================================================================
// XdrRef properties: offset, len, as_slice
// ===========================================================================

#[test]
fn ref_offset_and_len() {
    let val = ScVal::I32(42);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.offset(), 0);
    // i32 discriminant (4) + i32 value (4) = 8
    assert_eq!(r.len(), 8);
    assert_eq!(r.end(), 8);
    assert_eq!(r.as_slice(&buf).unwrap().len(), 8);
    assert_eq!(buf.len(), 8);
}

#[test]
fn ref_variable_length_value() {
    let val = ScVal::Bytes(ScBytes(vec![0u8; 13].try_into().unwrap()));
    let (_buf, r) = ref_at_zero(&val);
    // discriminant(4) + length_prefix(4) + data(13) + padding(3) = 24
    assert_eq!(r.len(), 24);
}

// ===========================================================================
// Validation failure on truncated / corrupt data
// ===========================================================================

#[test]
fn validate_truncated_buffer() {
    let val = ScVal::I32(42);
    let buf = val.to_xdr(Limits::none()).unwrap();
    // Truncate.
    let short = &buf[..4];
    let result = ScVal::xdr_ref_from(short, 0, &mut RefLimits::none());
    assert!(result.is_err());
}

#[test]
fn validate_empty_buffer() {
    let result = ScVal::xdr_ref_from(&[], 0, &mut RefLimits::none());
    assert!(result.is_err());
}

// ===========================================================================
// cast — reinterpret a typedef ref
// ===========================================================================

#[test]
fn cast_scvec_to_vecm() {
    let val = ScVec(
        vec![ScVal::I32(1), ScVal::I32(2)]
            .try_into()
            .unwrap(),
    );
    let (buf, r) = ref_at_zero(&val);

    // Cast from XdrRef<ScVec> to XdrRef<VecM<ScVal>> to access .count()
    let vecm_ref: XdrRef<VecM<ScVal>> = r.cast();
    assert_eq!(vecm_ref.count(&buf).unwrap(), 2);
}

// ===========================================================================
// UInt128Parts / Int128Parts standalone
// ===========================================================================

#[test]
fn uint128_parts_standalone() {
    let parts = UInt128Parts {
        hi: 0xFFFF_FFFF_FFFF_FFFF,
        lo: 0,
    };
    let (buf, r) = ref_at_zero(&parts);
    assert_eq!(r.hi(&buf).unwrap().get(&buf).unwrap(), u64::MAX);
    assert_eq!(r.lo(&buf).unwrap().get(&buf).unwrap(), 0);
}

#[test]
fn int128_parts_standalone() {
    let parts = Int128Parts {
        hi: i64::MIN,
        lo: u64::MAX,
    };
    let (buf, r) = ref_at_zero(&parts);
    assert_eq!(r.hi(&buf).unwrap().get(&buf).unwrap(), i64::MIN);
    assert_eq!(r.lo(&buf).unwrap().get(&buf).unwrap(), u64::MAX);
}

// ===========================================================================
// ScContractInstance
// ===========================================================================

#[test]
fn scval_contract_instance() {
    let instance = ScContractInstance {
        executable: ContractExecutable::StellarAsset,
        storage: None,
    };
    let val = ScVal::ContractInstance(instance.clone());
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(
        r.discriminant(&buf).unwrap(),
        ScValType::ContractInstance
    );
    let ci_ref = r.as_contract_instance(&buf).unwrap().unwrap();
    let exec_ref = ci_ref.executable(&buf).unwrap();
    assert_eq!(
        exec_ref.discriminant(&buf).unwrap(),
        ContractExecutableType::StellarAsset
    );
    let storage_ref = ci_ref.storage(&buf).unwrap();
    assert!(storage_ref.inner(&buf).unwrap().is_none());
    assert_eq!(ci_ref.materialize(&buf).unwrap(), instance);
}

#[test]
fn scval_contract_instance_with_storage() {
    let storage_map = ScMap(
        vec![ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::try_from("balance").unwrap())),
            val: ScVal::I128(Int128Parts { hi: 0, lo: 500 }),
        }]
        .try_into()
        .unwrap(),
    );
    let instance = ScContractInstance {
        executable: ContractExecutable::StellarAsset,
        storage: Some(storage_map),
    };
    let val = ScVal::ContractInstance(instance.clone());
    let (buf, r) = ref_at_zero(&val);
    let ci_ref = r.as_contract_instance(&buf).unwrap().unwrap();
    let storage_ref = ci_ref.storage(&buf).unwrap();
    let map_ref = storage_ref.inner(&buf).unwrap().unwrap();
    let mat = map_ref.materialize(&buf).unwrap();
    assert_eq!(mat.0.len(), 1);
    assert_eq!(ci_ref.materialize(&buf).unwrap(), instance);
}

// ===========================================================================
// Large buffer with many heterogeneous values
// ===========================================================================

#[test]
fn large_buffer_many_values() {
    let mut buf = Vec::new();
    let mut refs: Vec<XdrRef<ScVal>> = Vec::new();

    // Push 100 values of varying types.
    for i in 0u32..100 {
        let val = match i % 5 {
            0 => ScVal::I32(i as i32),
            1 => ScVal::U64(i as u64 * 1000),
            2 => ScVal::Bool(i % 2 == 0),
            3 => ScVal::Void,
            4 => ScVal::Symbol(ScSymbol(
                StringM::try_from(format!("s{i}").as_str()).unwrap(),
            )),
            _ => unreachable!(),
        };
        refs.push(push(&mut buf, &val));
    }

    // Spot-check some values.
    assert_eq!(
        refs[0].as_i32(&buf).unwrap().unwrap().get(&buf).unwrap(),
        0
    );
    assert_eq!(
        refs[6]
            .as_u64(&buf)
            .unwrap()
            .unwrap()
            .get(&buf)
            .unwrap(),
        6000
    );
    assert_eq!(
        refs[3].discriminant(&buf).unwrap(),
        ScValType::Void
    );
    assert_eq!(
        refs[99]
            .as_symbol(&buf)
            .unwrap()
            .unwrap()
            .materialize(&buf)
            .unwrap()
            .to_string(),
        "s99"
    );
}

// ===========================================================================
// ScAddress union
// ===========================================================================

#[test]
fn scval_address_contract() {
    let hash = Hash([0x42; 32]);
    let addr = ScAddress::Contract(ContractId(hash.clone()));
    let val = ScVal::Address(addr);
    let (buf, r) = ref_at_zero(&val);
    assert_eq!(r.discriminant(&buf).unwrap(), ScValType::Address);
    let addr_ref = r.as_address(&buf).unwrap().unwrap();
    assert_eq!(
        addr_ref.discriminant(&buf).unwrap(),
        ScAddressType::Contract
    );
    let mat = addr_ref.materialize(&buf).unwrap();
    if let ScAddress::Contract(h) = mat {
        assert_eq!(h.0 .0, [0x42; 32]);
    } else {
        panic!("expected Contract address");
    }
}
