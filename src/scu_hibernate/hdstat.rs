#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HDSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EPEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVR {
    #[doc = "Wake-up on positive edge pin event inactive"]
    VALUE1,
    #[doc = "Wake-up on positive edge pin event active"]
    VALUE2,
}
impl EPEVR {
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
            EPEVR::VALUE1 => false,
            EPEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPEVR {
        match value {
            false => EPEVR::VALUE1,
            true => EPEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EPEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EPEVR::VALUE2
    }
}
#[doc = "Possible values of the field `ENEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENEVR {
    #[doc = "Wake-up on negative edge pin event inactive"]
    VALUE1,
    #[doc = "Wake-up on negative edge pin event active"]
    VALUE2,
}
impl ENEVR {
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
            ENEVR::VALUE1 => false,
            ENEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENEVR {
        match value {
            false => ENEVR::VALUE1,
            true => ENEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENEVR::VALUE2
    }
}
#[doc = "Possible values of the field `RTCEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEVR {
    #[doc = "Wake-up on RTC event inactive"]
    VALUE1,
    #[doc = "Wake-up on RTC event active"]
    VALUE2,
}
impl RTCEVR {
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
            RTCEVR::VALUE1 => false,
            RTCEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCEVR {
        match value {
            false => RTCEVR::VALUE1,
            true => RTCEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTCEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTCEVR::VALUE2
    }
}
#[doc = "Possible values of the field `ULPWDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGR {
    #[doc = "Watchdog alarm did not occur"]
    VALUE1,
    #[doc = "Watchdog alarm occurred"]
    VALUE2,
}
impl ULPWDGR {
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
            ULPWDGR::VALUE1 => false,
            ULPWDGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGR {
        match value {
            false => ULPWDGR::VALUE1,
            true => ULPWDGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGR::VALUE2
    }
}
#[doc = "Possible values of the field `HIBNOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNOUTR {
    #[doc = "Hibernate not driven active to pads"]
    VALUE1,
    #[doc = "Hibernate driven active to pads"]
    VALUE2,
}
impl HIBNOUTR {
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
            HIBNOUTR::VALUE1 => false,
            HIBNOUTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBNOUTR {
        match value {
            false => HIBNOUTR::VALUE1,
            true => HIBNOUTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBNOUTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBNOUTR::VALUE2
    }
}
#[doc = "Possible values of the field `VBATPEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATPEVR {
    #[doc = "Wake-up on rising above threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on rising above threshold event active"]
    VALUE2,
}
impl VBATPEVR {
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
            VBATPEVR::VALUE1 => false,
            VBATPEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATPEVR {
        match value {
            false => VBATPEVR::VALUE1,
            true => VBATPEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBATPEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBATPEVR::VALUE2
    }
}
#[doc = "Possible values of the field `VBATNEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATNEVR {
    #[doc = "Wake-up on falling below threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on falling below threshold event active"]
    VALUE2,
}
impl VBATNEVR {
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
            VBATNEVR::VALUE1 => false,
            VBATNEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATNEVR {
        match value {
            false => VBATNEVR::VALUE1,
            true => VBATNEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBATNEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBATNEVR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO0PEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0PEVR {
    #[doc = "Wake-up on rising above threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on rising above threshold event active"]
    VALUE2,
}
impl AHIBIO0PEVR {
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
            AHIBIO0PEVR::VALUE1 => false,
            AHIBIO0PEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO0PEVR {
        match value {
            false => AHIBIO0PEVR::VALUE1,
            true => AHIBIO0PEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0PEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0PEVR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO0NEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0NEVR {
    #[doc = "Wake-up on falling below threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on falling below threshold event active"]
    VALUE2,
}
impl AHIBIO0NEVR {
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
            AHIBIO0NEVR::VALUE1 => false,
            AHIBIO0NEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO0NEVR {
        match value {
            false => AHIBIO0NEVR::VALUE1,
            true => AHIBIO0NEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0NEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0NEVR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO1PEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO1PEVR {
    #[doc = "Wake-up on rising above threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on rising above threshold event active"]
    VALUE2,
}
impl AHIBIO1PEVR {
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
            AHIBIO1PEVR::VALUE1 => false,
            AHIBIO1PEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO1PEVR {
        match value {
            false => AHIBIO1PEVR::VALUE1,
            true => AHIBIO1PEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1PEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1PEVR::VALUE2
    }
}
#[doc = "Possible values of the field `AHIBIO1NEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO1NEVR {
    #[doc = "Wake-up on falling below threshold event inactive"]
    VALUE1,
    #[doc = "Wake-up on falling below threshold event active"]
    VALUE2,
}
impl AHIBIO1NEVR {
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
            AHIBIO1NEVR::VALUE1 => false,
            AHIBIO1NEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHIBIO1NEVR {
        match value {
            false => AHIBIO1NEVR::VALUE1,
            true => AHIBIO1NEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1NEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1NEVR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline]
    pub fn epev(&self) -> EPEVR {
        EPEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline]
    pub fn enev(&self) -> ENEVR {
        ENEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline]
    pub fn rtcev(&self) -> RTCEVR {
        RTCEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline]
    pub fn ulpwdg(&self) -> ULPWDGR {
        ULPWDGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline]
    pub fn hibnout(&self) -> HIBNOUTR {
        HIBNOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
    #[inline]
    pub fn vbatpev(&self) -> VBATPEVR {
        VBATPEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
    #[inline]
    pub fn vbatnev(&self) -> VBATNEVR {
        VBATNEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
    #[inline]
    pub fn ahibio0pev(&self) -> AHIBIO0PEVR {
        AHIBIO0PEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
    #[inline]
    pub fn ahibio0nev(&self) -> AHIBIO0NEVR {
        AHIBIO0NEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing"]
    #[inline]
    pub fn ahibio1pev(&self) -> AHIBIO1PEVR {
        AHIBIO1PEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing"]
    #[inline]
    pub fn ahibio1nev(&self) -> AHIBIO1NEVR {
        AHIBIO1NEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
