#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VLS1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS1SR {
    #[doc = "Value switch not detected"]
    VALUE1,
    #[doc = "Value switch detected"]
    VALUE2,
}
impl VLS1SR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VLS1SR::VALUE1 => false,
            VLS1SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLS1SR {
        match value {
            false => VLS1SR::VALUE1,
            true => VLS1SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VLS1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VLS1SR::VALUE2
    }
}
#[doc = "Possible values of the field `VLS2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS2SR {
    #[doc = "Value switch not detected"]
    VALUE1,
    #[doc = "Value switch detected"]
    VALUE2,
}
impl VLS2SR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VLS2SR::VALUE1 => false,
            VLS2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLS2SR {
        match value {
            false => VLS2SR::VALUE1,
            true => VLS2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VLS2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VLS2SR::VALUE2
    }
}
#[doc = "Possible values of the field `TRGSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSSR {
    #[doc = "Conversion trigger was not generated"]
    VALUE1,
    #[doc = "Conversion trigger was generated"]
    VALUE2,
}
impl TRGSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRGSSR::VALUE1 => false,
            TRGSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGSSR {
        match value {
            false => TRGSSR::VALUE1,
            true => TRGSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRGSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRGSSR::VALUE2
    }
}
#[doc = "Possible values of the field `STRSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRSSR {
    #[doc = "Start trigger not detected"]
    VALUE1,
    #[doc = "Start trigger detected"]
    VALUE2,
}
impl STRSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STRSSR::VALUE1 => false,
            STRSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRSSR {
        match value {
            false => STRSSR::VALUE1,
            true => STRSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STRSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STRSSR::VALUE2
    }
}
#[doc = "Possible values of the field `STPSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPSSR {
    #[doc = "Stop trigger not detected"]
    VALUE1,
    #[doc = "Stop trigger detected"]
    VALUE2,
}
impl STPSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STPSSR::VALUE1 => false,
            STPSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPSSR {
        match value {
            false => STPSSR::VALUE1,
            true => STPSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STPSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STPSSR::VALUE2
    }
}
#[doc = "Possible values of the field `STDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDSR {
    #[doc = "Shadow transfer was not performed"]
    VALUE1,
    #[doc = "Shadow transfer was performed"]
    VALUE2,
}
impl STDSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STDSR::VALUE1 => false,
            STDSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STDSR {
        match value {
            false => STDSR::VALUE1,
            true => STDSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STDSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STDSR::VALUE2
    }
}
#[doc = "Possible values of the field `CRSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSSR {
    #[doc = "Comparator output LOW to HIGH transition not detected"]
    VALUE1,
    #[doc = "Comparator output LOW to HIGH transition detected"]
    VALUE2,
}
impl CRSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CRSSR::VALUE1 => false,
            CRSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRSSR {
        match value {
            false => CRSSR::VALUE1,
            true => CRSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CRSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CRSSR::VALUE2
    }
}
#[doc = "Possible values of the field `CFSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFSSR {
    #[doc = "Comparator output HIGH to LOW transition not detected"]
    VALUE1,
    #[doc = "Comparator output HIGH to LOW transition detected"]
    VALUE2,
}
impl CFSSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CFSSR::VALUE1 => false,
            CFSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFSSR {
        match value {
            false => CFSSR::VALUE1,
            true => CFSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFSSR::VALUE2
    }
}
#[doc = "Possible values of the field `CSES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSESR {
    #[doc = "Comparator output has been set to the clamped state"]
    VALUE1,
    #[doc = "Comparator output has not been set to the clamped state"]
    VALUE2,
}
impl CSESR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CSESR::VALUE1 => false,
            CSESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSESR {
        match value {
            false => CSESR::VALUE1,
            true => CSESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSESR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
    #[inline]
    pub fn vls1s(&self) -> VLS1SR {
        VLS1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
    #[inline]
    pub fn vls2s(&self) -> VLS2SR {
        VLS2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Conversion trigger status"]
    #[inline]
    pub fn trgss(&self) -> TRGSSR {
        TRGSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Start trigger interrupt status"]
    #[inline]
    pub fn strss(&self) -> STRSSR {
        STRSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stop trigger interrupt status"]
    #[inline]
    pub fn stpss(&self) -> STPSSR {
        STPSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Shadow transfer interrupt status"]
    #[inline]
    pub fn stds(&self) -> STDSR {
        STDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Comparator rise interrupt status"]
    #[inline]
    pub fn crss(&self) -> CRSSR {
        CRSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Comparator fall interrupt status"]
    #[inline]
    pub fn cfss(&self) -> CFSSR {
        CFSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Comparator clamped interrupt status"]
    #[inline]
    pub fn cses(&self) -> CSESR {
        CSESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
