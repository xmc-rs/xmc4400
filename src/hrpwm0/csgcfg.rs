#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSGCFG {
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
#[doc = "Possible values of the field `C0PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0PMR {
    #[doc = "CSG0 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG0 unit is set in High Speed Mode"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C0PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C0PMR::VALUE1 => 0,
            C0PMR::VALUE2 => 1,
            C0PMR::VALUE4 => 3,
            C0PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C0PMR {
        match value {
            0 => C0PMR::VALUE1,
            1 => C0PMR::VALUE2,
            3 => C0PMR::VALUE4,
            i => C0PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0PMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0PMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == C0PMR::VALUE4
    }
}
#[doc = "Possible values of the field `C1PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1PMR {
    #[doc = "CSG1 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG1 unit is set in High Speed Mode"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C1PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C1PMR::VALUE1 => 0,
            C1PMR::VALUE2 => 1,
            C1PMR::VALUE4 => 3,
            C1PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C1PMR {
        match value {
            0 => C1PMR::VALUE1,
            1 => C1PMR::VALUE2,
            3 => C1PMR::VALUE4,
            i => C1PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1PMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1PMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == C1PMR::VALUE4
    }
}
#[doc = "Possible values of the field `C2PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2PMR {
    #[doc = "CSG2 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG2 unit is set in High Speed Mode"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C2PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C2PMR::VALUE1 => 0,
            C2PMR::VALUE2 => 1,
            C2PMR::VALUE4 => 3,
            C2PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C2PMR {
        match value {
            0 => C2PMR::VALUE1,
            1 => C2PMR::VALUE2,
            3 => C2PMR::VALUE4,
            i => C2PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C2PMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C2PMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == C2PMR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct C0CDR {
    bits: bool,
}
impl C0CDR {
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
pub struct C1CDR {
    bits: bool,
}
impl C1CDR {
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
pub struct C2CDR {
    bits: bool,
}
impl C2CDR {
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
#[doc = "Values that can be written to the field `C0PM`"]
pub enum C0PMW {
    #[doc = "CSG0 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG0 unit is set in High Speed Mode"]
    VALUE4,
}
impl C0PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C0PMW::VALUE1 => 0,
            C0PMW::VALUE2 => 1,
            C0PMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0PMW<'a> {
    w: &'a mut W,
}
impl<'a> _C0PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CSG0 unit is powered OFF"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0PMW::VALUE1)
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0PMW::VALUE2)
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(C0PMW::VALUE4)
    }
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
#[doc = "Values that can be written to the field `C1PM`"]
pub enum C1PMW {
    #[doc = "CSG1 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG1 unit is set in High Speed Mode"]
    VALUE4,
}
impl C1PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C1PMW::VALUE1 => 0,
            C1PMW::VALUE2 => 1,
            C1PMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1PMW<'a> {
    w: &'a mut W,
}
impl<'a> _C1PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CSG1 unit is powered OFF"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1PMW::VALUE1)
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1PMW::VALUE2)
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(C1PMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C2PM`"]
pub enum C2PMW {
    #[doc = "CSG2 unit is powered OFF"]
    VALUE1,
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "CSG2 unit is set in High Speed Mode"]
    VALUE4,
}
impl C2PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C2PMW::VALUE1 => 0,
            C2PMW::VALUE2 => 1,
            C2PMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C2PMW<'a> {
    w: &'a mut W,
}
impl<'a> _C2PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C2PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CSG2 unit is powered OFF"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C2PMW::VALUE1)
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C2PMW::VALUE2)
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(C2PMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C0CDW<'a> {
    w: &'a mut W,
}
impl<'a> _C0CDW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C1CDW<'a> {
    w: &'a mut W,
}
impl<'a> _C1CDW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C2CDW<'a> {
    w: &'a mut W,
}
impl<'a> _C2CDW<'a> {
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
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline]
    pub fn c0pm(&self) -> C0PMR {
        C0PMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline]
    pub fn c1pm(&self) -> C1PMR {
        C1PMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline]
    pub fn c2pm(&self) -> C2PMR {
        C2PMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline]
    pub fn c0cd(&self) -> C0CDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C0CDR { bits }
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline]
    pub fn c1cd(&self) -> C1CDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C1CDR { bits }
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline]
    pub fn c2cd(&self) -> C2CDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C2CDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline]
    pub fn c0pm(&mut self) -> _C0PMW {
        _C0PMW { w: self }
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline]
    pub fn c1pm(&mut self) -> _C1PMW {
        _C1PMW { w: self }
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline]
    pub fn c2pm(&mut self) -> _C2PMW {
        _C2PMW { w: self }
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline]
    pub fn c0cd(&mut self) -> _C0CDW {
        _C0CDW { w: self }
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline]
    pub fn c1cd(&mut self) -> _C1CDW {
        _C1CDW { w: self }
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline]
    pub fn c2cd(&mut self) -> _C2CDW {
        _C2CDW { w: self }
    }
}
