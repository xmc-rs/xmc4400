#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LPACST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VBATSCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSCMPR {
    #[doc = "Ready to start new compare operation"]
    VALUE1,
    #[doc = "Compare operation completed"]
    VALUE2,
}
impl VBATSCMPR {
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
            VBATSCMPR::VALUE1 => false,
            VBATSCMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATSCMPR {
        match value {
            false => VBATSCMPR::VALUE1,
            true => VBATSCMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBATSCMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBATSCMPR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO0SCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0SCMPR {
    #[doc = "Ready to start new compare operation"]
    VALUE1,
    #[doc = "Compare operation completed"]
    VALUE2,
}
impl AHIBIO0SCMPR {
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
            AHIBIO0SCMPR::VALUE1 => false,
            AHIBIO0SCMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO0SCMPR {
        match value {
            false => AHIBIO0SCMPR::VALUE1,
            true => AHIBIO0SCMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0SCMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0SCMPR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO1SCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO1SCMPR {
    #[doc = "Ready to start new compare operation"]
    VALUE1,
    #[doc = "Compare operation completed"]
    VALUE2,
}
impl AHIBIO1SCMPR {
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
            AHIBIO1SCMPR::VALUE1 => false,
            AHIBIO1SCMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO1SCMPR {
        match value {
            false => AHIBIO1SCMPR::VALUE1,
            true => AHIBIO1SCMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1SCMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1SCMPR::VALUE2
    }
}
#[doc = "Possible values of the field `VBATVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATVALR {
    #[doc = "Below programmed threshold"]
    VALUE1,
    #[doc = "Above programmed threshold"]
    VALUE2,
}
impl VBATVALR {
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
            VBATVALR::VALUE1 => false,
            VBATVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATVALR {
        match value {
            false => VBATVALR::VALUE1,
            true => VBATVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBATVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBATVALR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO0VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0VALR {
    #[doc = "Below programmed threshold"]
    VALUE1,
    #[doc = "Above programmed threshold"]
    VALUE2,
}
impl AHIBIO0VALR {
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
            AHIBIO0VALR::VALUE1 => false,
            AHIBIO0VALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO0VALR {
        match value {
            false => AHIBIO0VALR::VALUE1,
            true => AHIBIO0VALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0VALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0VALR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO1VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO1VALR {
    #[doc = "Below programmed threshold"]
    VALUE1,
    #[doc = "Above programmed threshold"]
    VALUE2,
}
impl AHIBIO1VALR {
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
            AHIBIO1VALR::VALUE1 => false,
            AHIBIO1VALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO1VALR {
        match value {
            false => AHIBIO1VALR::VALUE1,
            true => AHIBIO1VALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1VALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1VALR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Status"]
    #[inline]
    pub fn vbatscmp(&self) -> VBATSCMPR {
        VBATSCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Status"]
    #[inline]
    pub fn ahibio0scmp(&self) -> AHIBIO0SCMPR {
        AHIBIO0SCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Trigger HIB_IO_1 Input Single Compare Operation Status"]
    #[inline]
    pub fn ahibio1scmp(&self) -> AHIBIO1SCMPR {
        AHIBIO1SCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - VBAT Compare Operation Result"]
    #[inline]
    pub fn vbatval(&self) -> VBATVALR {
        VBATVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Operation Result"]
    #[inline]
    pub fn ahibio0val(&self) -> AHIBIO0VALR {
        AHIBIO0VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - HIB_IO_1 Input Compare Operation Result"]
    #[inline]
    pub fn ahibio1val(&self) -> AHIBIO1VALR {
        AHIBIO1VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
