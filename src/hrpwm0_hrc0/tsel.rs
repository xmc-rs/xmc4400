#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSEL {
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
#[doc = "Possible values of the field `TSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL0R {
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSEL0R::VALUE1 => 0,
            TSEL0R::VALUE2 => 1,
            TSEL0R::VALUE3 => 2,
            TSEL0R::VALUE4 => 3,
            TSEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSEL0R {
        match value {
            0 => TSEL0R::VALUE1,
            1 => TSEL0R::VALUE2,
            2 => TSEL0R::VALUE3,
            3 => TSEL0R::VALUE4,
            i => TSEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSEL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSEL0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TSEL0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TSEL0R::VALUE4
    }
}
#[doc = "Possible values of the field `TSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL1R {
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSEL1R::VALUE1 => 0,
            TSEL1R::VALUE2 => 1,
            TSEL1R::VALUE3 => 2,
            TSEL1R::VALUE4 => 3,
            TSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSEL1R {
        match value {
            0 => TSEL1R::VALUE1,
            1 => TSEL1R::VALUE2,
            2 => TSEL1R::VALUE3,
            3 => TSEL1R::VALUE4,
            i => TSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSEL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSEL1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TSEL1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TSEL1R::VALUE4
    }
}
#[doc = "Possible values of the field `TS0E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS0ER {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    VALUE1,
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    VALUE2,
}
impl TS0ER {
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
            TS0ER::VALUE1 => false,
            TS0ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TS0ER {
        match value {
            false => TS0ER::VALUE1,
            true => TS0ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TS0ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TS0ER::VALUE2
    }
}
#[doc = "Possible values of the field `TS1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS1ER {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    VALUE1,
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    VALUE2,
}
impl TS1ER {
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
            TS1ER::VALUE1 => false,
            TS1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TS1ER {
        match value {
            false => TS1ER::VALUE1,
            true => TS1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TS1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TS1ER::VALUE2
    }
}
#[doc = "Values that can be written to the field `TSEL0`"]
pub enum TSEL0W {
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
}
impl TSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSEL0W::VALUE1 => 0,
            TSEL0W::VALUE2 => 1,
            TSEL0W::VALUE3 => 2,
            TSEL0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL0W::VALUE1)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL0W::VALUE2)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL0W::VALUE3)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL0W::VALUE4)
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
#[doc = "Values that can be written to the field `TSEL1`"]
pub enum TSEL1W {
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
}
impl TSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSEL1W::VALUE1 => 0,
            TSEL1W::VALUE2 => 1,
            TSEL1W::VALUE3 => 2,
            TSEL1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL1W::VALUE1)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL1W::VALUE2)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL1W::VALUE3)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TS0E`"]
pub enum TS0EW {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    VALUE1,
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    VALUE2,
}
impl TS0EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TS0EW::VALUE1 => false,
            TS0EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TS0EW<'a> {
    w: &'a mut W,
}
impl<'a> _TS0EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TS0EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS0EW::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS0EW::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TS1E`"]
pub enum TS1EW {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    VALUE1,
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    VALUE2,
}
impl TS1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TS1EW::VALUE1 => false,
            TS1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TS1EW<'a> {
    w: &'a mut W,
}
impl<'a> _TS1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TS1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS1EW::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS1EW::VALUE2)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline]
    pub fn tsel0(&self) -> TSEL0R {
        TSEL0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline]
    pub fn tsel1(&self) -> TSEL1R {
        TSEL1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline]
    pub fn ts0e(&self) -> TS0ER {
        TS0ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline]
    pub fn ts1e(&self) -> TS1ER {
        TS1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline]
    pub fn tsel0(&mut self) -> _TSEL0W {
        _TSEL0W { w: self }
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline]
    pub fn tsel1(&mut self) -> _TSEL1W {
        _TSEL1W { w: self }
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline]
    pub fn ts0e(&mut self) -> _TS0EW {
        _TS0EW { w: self }
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline]
    pub fn ts1e(&mut self) -> _TS1EW {
        _TS1EW { w: self }
    }
}
