use crate::impl_number_type;
impl_number_type!(u32, UnsignedInteger, AsUnsignedInteger, as_unsigned_integer);

impl From<u64> for UnsignedInteger {
    fn from(value: u64) -> UnsignedInteger {
        if value <= u32::MAX.into() {
            UnsignedInteger { value: value as u32 }
        } else {
            panic!("cannot convert from {:#?} to {}", value, UnsignedInteger::type_name())
        }
    }
}

impl AsNumber<u64> for u32 {
    fn as_number(&self) -> u64 {
        *self as u64
    }
}

impl AsUnsignedInteger for u32 {
    fn as_unsigned_integer(&self) -> UnsignedInteger {
        UnsignedInteger::from(*self as u64)
    }
}
