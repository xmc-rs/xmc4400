#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLBANA {
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
#[doc = r" Value of the field"]
pub struct SLDLYR {
    bits: u8,
}
impl SLDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FUPR {
    bits: bool,
}
impl FUPR {
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
pub struct FDNR {
    bits: bool,
}
impl FDNR {
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
pub struct SLCPR {
    bits: u8,
}
impl SLCPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLIBLDOR {
    bits: u8,
}
impl SLIBLDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLIBLFR {
    bits: u8,
}
impl SLIBLFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLVREFR {
    bits: u8,
}
impl SLVREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIBIASR {
    bits: u8,
}
impl TRIBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GHREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GHRENR {
    #[doc = "Global high resolution generation is enabled"]
    VALUE1,
    #[doc = "Global high resolution generation is disabled"]
    VALUE2,
}
impl GHRENR {
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
            GHRENR::VALUE1 => false,
            GHRENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GHRENR {
        match value {
            false => GHRENR::VALUE1,
            true => GHRENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GHRENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GHRENR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _SLDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SLDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FUPW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FDNW<'a> {
    w: &'a mut W,
}
impl<'a> _FDNW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLCPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLCPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLIBLDOW<'a> {
    w: &'a mut W,
}
impl<'a> _SLIBLDOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLIBLFW<'a> {
    w: &'a mut W,
}
impl<'a> _SLIBLFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLVREFW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVREFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIBIASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GHREN`"]
pub enum GHRENW {
    #[doc = "Global high resolution generation is enabled"]
    VALUE1,
    #[doc = "Global high resolution generation is disabled"]
    VALUE2,
}
impl GHRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GHRENW::VALUE1 => false,
            GHRENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GHRENW<'a> {
    w: &'a mut W,
}
impl<'a> _GHRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GHRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Global high resolution generation is enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GHRENW::VALUE1)
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GHRENW::VALUE2)
    }
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline]
    pub fn sldly(&self) -> SLDLYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLDLYR { bits }
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline]
    pub fn fup(&self) -> FUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FUPR { bits }
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline]
    pub fn fdn(&self) -> FDNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FDNR { bits }
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline]
    pub fn slcp(&self) -> SLCPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLCPR { bits }
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline]
    pub fn slibldo(&self) -> SLIBLDOR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLIBLDOR { bits }
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline]
    pub fn sliblf(&self) -> SLIBLFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLIBLFR { bits }
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline]
    pub fn slvref(&self) -> SLVREFR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLVREFR { bits }
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline]
    pub fn tribias(&self) -> TRIBIASR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIBIASR { bits }
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline]
    pub fn ghren(&self) -> GHRENR {
        GHRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 19340 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline]
    pub fn sldly(&mut self) -> _SLDLYW {
        _SLDLYW { w: self }
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline]
    pub fn fup(&mut self) -> _FUPW {
        _FUPW { w: self }
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline]
    pub fn fdn(&mut self) -> _FDNW {
        _FDNW { w: self }
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline]
    pub fn slcp(&mut self) -> _SLCPW {
        _SLCPW { w: self }
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline]
    pub fn slibldo(&mut self) -> _SLIBLDOW {
        _SLIBLDOW { w: self }
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline]
    pub fn sliblf(&mut self) -> _SLIBLFW {
        _SLIBLFW { w: self }
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline]
    pub fn slvref(&mut self) -> _SLVREFW {
        _SLVREFW { w: self }
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline]
    pub fn tribias(&mut self) -> _TRIBIASW {
        _TRIBIASW { w: self }
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline]
    pub fn ghren(&mut self) -> _GHRENW {
        _GHRENW { w: self }
    }
}
