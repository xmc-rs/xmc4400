#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPACCLR {
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
}
#[doc = "Values that can be written to the field `VBATSCMP`"]
pub enum VBATSCMPW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the sticky bit"]
    VALUE2,
}
impl VBATSCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATSCMPW::VALUE1 => false,
            VBATSCMPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATSCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATSCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATSCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATSCMPW::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATSCMPW::VALUE2)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHIBIO0SCMP`"]
pub enum AHIBIO0SCMPW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the sticky bit"]
    VALUE2,
}
impl AHIBIO0SCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO0SCMPW::VALUE1 => false,
            AHIBIO0SCMPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO0SCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO0SCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO0SCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0SCMPW::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0SCMPW::VALUE2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHIBIO1SCMP`"]
pub enum AHIBIO1SCMPW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the sticky bit"]
    VALUE2,
}
impl AHIBIO1SCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO1SCMPW::VALUE1 => false,
            AHIBIO1SCMPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO1SCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO1SCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO1SCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1SCMPW::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1SCMPW::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBATVAL`"]
pub enum VBATVALW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Below programmed threshold"]
    VALUE2,
}
impl VBATVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATVALW::VALUE1 => false,
            VBATVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATVALW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATVALW::VALUE1)
    }
    #[doc = "Below programmed threshold"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATVALW::VALUE2)
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
#[doc = "Values that can be written to the field `AHIBIO0VAL`"]
pub enum AHIBIO0VALW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Below programmed threshold"]
    VALUE2,
}
impl AHIBIO0VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO0VALW::VALUE1 => false,
            AHIBIO0VALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO0VALW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO0VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO0VALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0VALW::VALUE1)
    }
    #[doc = "Below programmed threshold"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0VALW::VALUE2)
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
#[doc = "Values that can be written to the field `AHIBIO1VAL`"]
pub enum AHIBIO1VALW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Below programmed threshold"]
    VALUE2,
}
impl AHIBIO1VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO1VALW::VALUE1 => false,
            AHIBIO1VALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO1VALW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO1VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO1VALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1VALW::VALUE1)
    }
    #[doc = "Below programmed threshold"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1VALW::VALUE2)
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
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Clear"]
    #[inline]
    pub fn vbatscmp(&mut self) -> _VBATSCMPW {
        _VBATSCMPW { w: self }
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
    #[inline]
    pub fn ahibio0scmp(&mut self) -> _AHIBIO0SCMPW {
        _AHIBIO0SCMPW { w: self }
    }
    #[doc = "Bit 2 - Trigger HIB_IO_1 Input Single Compare Operation Clear"]
    #[inline]
    pub fn ahibio1scmp(&mut self) -> _AHIBIO1SCMPW {
        _AHIBIO1SCMPW { w: self }
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Clear"]
    #[inline]
    pub fn vbatval(&mut self) -> _VBATVALW {
        _VBATVALW { w: self }
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Clear"]
    #[inline]
    pub fn ahibio0val(&mut self) -> _AHIBIO0VALW {
        _AHIBIO0VALW { w: self }
    }
    #[doc = "Bit 18 - HIB_IO_1 Input Compare Initial Value Clear"]
    #[inline]
    pub fn ahibio1val(&mut self) -> _AHIBIO1VALW {
        _AHIBIO1VALW { w: self }
    }
}
