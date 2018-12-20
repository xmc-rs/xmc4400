#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSGSTATG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `D0RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0RBR {
    #[doc = "DAC0 is not running (control logic is disabled)"]
    VALUE1,
    #[doc = "DAC0 is running"]
    VALUE2,
}
impl D0RBR {
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
            D0RBR::VALUE1 => false,
            D0RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D0RBR {
        match value {
            false => D0RBR::VALUE1,
            true => D0RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D0RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D0RBR::VALUE2
    }
}
#[doc = "Possible values of the field `C0RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0RBR {
    #[doc = "CMP0 functionality is disabled"]
    VALUE1,
    #[doc = "CMP0 functionality is enabled"]
    VALUE2,
}
impl C0RBR {
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
            C0RBR::VALUE1 => false,
            C0RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C0RBR {
        match value {
            false => C0RBR::VALUE1,
            true => C0RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0RBR::VALUE2
    }
}
#[doc = "Possible values of the field `PSLS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSLS0R {
    #[doc = "CMP0 output is not clamped"]
    VALUE1,
    #[doc = "CMP0 output is clamped"]
    VALUE2,
}
impl PSLS0R {
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
            PSLS0R::VALUE1 => false,
            PSLS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSLS0R {
        match value {
            false => PSLS0R::VALUE1,
            true => PSLS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSLS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSLS0R::VALUE2
    }
}
#[doc = "Possible values of the field `D1RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1RBR {
    #[doc = "DAC1 is not running (control logic is disabled)"]
    VALUE1,
    #[doc = "DAC1 is running"]
    VALUE2,
}
impl D1RBR {
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
            D1RBR::VALUE1 => false,
            D1RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D1RBR {
        match value {
            false => D1RBR::VALUE1,
            true => D1RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D1RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D1RBR::VALUE2
    }
}
#[doc = "Possible values of the field `C1RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1RBR {
    #[doc = "CMP1 functionality is disabled"]
    VALUE1,
    #[doc = "CMP1 functionality is enabled"]
    VALUE2,
}
impl C1RBR {
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
            C1RBR::VALUE1 => false,
            C1RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C1RBR {
        match value {
            false => C1RBR::VALUE1,
            true => C1RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1RBR::VALUE2
    }
}
#[doc = "Possible values of the field `PSLS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSLS1R {
    #[doc = "CMP1 output is not clamped"]
    VALUE1,
    #[doc = "CMP1 output is clamped"]
    VALUE2,
}
impl PSLS1R {
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
            PSLS1R::VALUE1 => false,
            PSLS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSLS1R {
        match value {
            false => PSLS1R::VALUE1,
            true => PSLS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSLS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSLS1R::VALUE2
    }
}
#[doc = "Possible values of the field `D2RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2RBR {
    #[doc = "DAC2 is not running (control logic is disabled)"]
    VALUE1,
    #[doc = "DAC1 is running"]
    VALUE2,
}
impl D2RBR {
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
            D2RBR::VALUE1 => false,
            D2RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D2RBR {
        match value {
            false => D2RBR::VALUE1,
            true => D2RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D2RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D2RBR::VALUE2
    }
}
#[doc = "Possible values of the field `C2RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2RBR {
    #[doc = "CMP2 functionality is disabled"]
    VALUE1,
    #[doc = "CMP2 functionality is enabled"]
    VALUE2,
}
impl C2RBR {
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
            C2RBR::VALUE1 => false,
            C2RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C2RBR {
        match value {
            false => C2RBR::VALUE1,
            true => C2RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C2RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C2RBR::VALUE2
    }
}
#[doc = "Possible values of the field `PSLS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSLS2R {
    #[doc = "CMP2 output is not clamped"]
    VALUE1,
    #[doc = "CMP2 output is clamped"]
    VALUE2,
}
impl PSLS2R {
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
            PSLS2R::VALUE1 => false,
            PSLS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSLS2R {
        match value {
            false => PSLS2R::VALUE1,
            true => PSLS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSLS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSLS2R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DAC0 run bit status"]
    #[inline]
    pub fn d0rb(&self) -> D0RBR {
        D0RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CMP0 run bit status"]
    #[inline]
    pub fn c0rb(&self) -> C0RBR {
        C0RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CMP0 output passive status"]
    #[inline]
    pub fn psls0(&self) -> PSLS0R {
        PSLS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DAC1 run bit status"]
    #[inline]
    pub fn d1rb(&self) -> D1RBR {
        D1RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CMP1 run bit status"]
    #[inline]
    pub fn c1rb(&self) -> C1RBR {
        C1RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CMP1 output passive status"]
    #[inline]
    pub fn psls1(&self) -> PSLS1R {
        PSLS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - DAC2 run bit status"]
    #[inline]
    pub fn d2rb(&self) -> D2RBR {
        D2RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CMP2 run bit status"]
    #[inline]
    pub fn c2rb(&self) -> C2RBR {
        C2RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CMP2 output passive status"]
    #[inline]
    pub fn psls2(&self) -> PSLS2R {
        PSLS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
