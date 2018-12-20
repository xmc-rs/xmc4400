#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPACCONF {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CMPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPENR {
    #[doc = "Comparator permanently in power down"]
    VALUE1,
    #[doc = "Comparator activated for VBAT input"]
    VALUE2,
    #[doc = "Comparator activated for HIB_IO_0 input"]
    VALUE3,
    #[doc = "Comparator activated for HIB_IO_1 input"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPENR::VALUE1 => 0,
            CMPENR::VALUE2 => 1,
            CMPENR::VALUE3 => 2,
            CMPENR::VALUE4 => 4,
            CMPENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPENR {
        match value {
            0 => CMPENR::VALUE1,
            1 => CMPENR::VALUE2,
            2 => CMPENR::VALUE3,
            4 => CMPENR::VALUE4,
            i => CMPENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMPENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMPENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMPENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMPENR::VALUE4
    }
}
#[doc = "Possible values of the field `TRIGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSELR {
    #[doc = "Sub-second interval counter"]
    VALUE1,
    #[doc = "RTC alarm event"]
    VALUE2,
    #[doc = "RTC periodic event"]
    VALUE3,
    #[doc = "On digital WKUP input positive edge event"]
    VALUE4,
    #[doc = "On digital WKUP input negative edge event"]
    VALUE5,
    #[doc = "Continuous measurement"]
    VALUE6,
    #[doc = "Single-shot on software request"]
    VALUE7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSELR::VALUE1 => 0,
            TRIGSELR::VALUE2 => 1,
            TRIGSELR::VALUE3 => 2,
            TRIGSELR::VALUE4 => 3,
            TRIGSELR::VALUE5 => 5,
            TRIGSELR::VALUE6 => 6,
            TRIGSELR::VALUE7 => 7,
            TRIGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSELR {
        match value {
            0 => TRIGSELR::VALUE1,
            1 => TRIGSELR::VALUE2,
            2 => TRIGSELR::VALUE3,
            3 => TRIGSELR::VALUE4,
            5 => TRIGSELR::VALUE5,
            6 => TRIGSELR::VALUE6,
            7 => TRIGSELR::VALUE7,
            i => TRIGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRIGSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRIGSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRIGSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRIGSELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == TRIGSELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == TRIGSELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == TRIGSELR::VALUE7
    }
}
#[doc = r" Value of the field"]
pub struct CONVDELR {
    bits: bool,
}
impl CONVDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct INTERVCNTR {
    bits: u16,
}
impl INTERVCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SETTLECNTR {
    bits: u8,
}
impl SETTLECNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CMPEN`"]
pub enum CMPENW {
    #[doc = "Comparator permanently in power down"]
    VALUE1,
    #[doc = "Comparator activated for VBAT input"]
    VALUE2,
    #[doc = "Comparator activated for HIB_IO_0 input"]
    VALUE3,
    #[doc = "Comparator activated for HIB_IO_1 input"]
    VALUE4,
}
impl CMPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPENW::VALUE1 => 0,
            CMPENW::VALUE2 => 1,
            CMPENW::VALUE3 => 2,
            CMPENW::VALUE4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Comparator permanently in power down"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMPENW::VALUE1)
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMPENW::VALUE2)
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMPENW::VALUE3)
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMPENW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGSEL`"]
pub enum TRIGSELW {
    #[doc = "Sub-second interval counter"]
    VALUE1,
    #[doc = "RTC alarm event"]
    VALUE2,
    #[doc = "RTC periodic event"]
    VALUE3,
    #[doc = "On digital WKUP input positive edge event"]
    VALUE4,
    #[doc = "On digital WKUP input negative edge event"]
    VALUE5,
    #[doc = "Continuous measurement"]
    VALUE6,
    #[doc = "Single-shot on software request"]
    VALUE7,
}
impl TRIGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSELW::VALUE1 => 0,
            TRIGSELW::VALUE2 => 1,
            TRIGSELW::VALUE3 => 2,
            TRIGSELW::VALUE4 => 3,
            TRIGSELW::VALUE5 => 5,
            TRIGSELW::VALUE6 => 6,
            TRIGSELW::VALUE7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sub-second interval counter"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE1)
    }
    #[doc = "RTC alarm event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE2)
    }
    #[doc = "RTC periodic event"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE3)
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE4)
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE5)
    }
    #[doc = "Continuous measurement"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE6)
    }
    #[doc = "Single-shot on software request"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(TRIGSELW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONVDELW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVDELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTERVCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERVCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SETTLECNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SETTLECNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline]
    pub fn cmpen(&self) -> CMPENR {
        CMPENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline]
    pub fn trigsel(&self) -> TRIGSELR {
        TRIGSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline]
    pub fn convdel(&self) -> CONVDELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONVDELR { bits }
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline]
    pub fn intervcnt(&self) -> INTERVCNTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INTERVCNTR { bits }
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline]
    pub fn settlecnt(&self) -> SETTLECNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SETTLECNTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1879048192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline]
    pub fn cmpen(&mut self) -> _CMPENW {
        _CMPENW { w: self }
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline]
    pub fn trigsel(&mut self) -> _TRIGSELW {
        _TRIGSELW { w: self }
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline]
    pub fn convdel(&mut self) -> _CONVDELW {
        _CONVDELW { w: self }
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline]
    pub fn intervcnt(&mut self) -> _INTERVCNTW {
        _INTERVCNTW { w: self }
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline]
    pub fn settlecnt(&mut self) -> _SETTLECNTW {
        _SETTLECNTW { w: self }
    }
}
