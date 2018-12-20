#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRCLR {
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
#[doc = "Values that can be written to the field `PRWARN`"]
pub enum PRWARNW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl PRWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRWARNW::VALUE1 => false,
            PRWARNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _PRWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE2)
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
#[doc = "Values that can be written to the field `PI`"]
pub enum PIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl PIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIW::VALUE1 => false,
            PIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIW<'a> {
    w: &'a mut W,
}
impl<'a> _PIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PIW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PIW::VALUE2)
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
#[doc = "Values that can be written to the field `AI`"]
pub enum AIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIW::VALUE1 => false,
            AIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIW<'a> {
    w: &'a mut W,
}
impl<'a> _AIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIW::VALUE2)
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
#[doc = "Values that can be written to the field `DLROVR`"]
pub enum DLROVRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl DLROVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLROVRW::VALUE1 => false,
            DLROVRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLROVRW<'a> {
    w: &'a mut W,
}
impl<'a> _DLROVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLROVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLROVRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLROVRW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACCR`"]
pub enum LPACCRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACCRW::VALUE1 => false,
            LPACCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACCRW<'a> {
    w: &'a mut W,
}
impl<'a> _LPACCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCRW::VALUE2)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACTH0`"]
pub enum LPACTH0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACTH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACTH0W::VALUE1 => false,
            LPACTH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACTH0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPACTH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACTH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH0W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH0W::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACTH1`"]
pub enum LPACTH1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACTH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACTH1W::VALUE1 => false,
            LPACTH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACTH1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPACTH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACTH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH1W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH1W::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACST`"]
pub enum LPACSTW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACSTW::VALUE1 => false,
            LPACSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LPACSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACSTW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACSTW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACCLR`"]
pub enum LPACCLRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACCLRW::VALUE1 => false,
            LPACCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _LPACCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCLRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCLRW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPACSET`"]
pub enum LPACSETW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl LPACSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPACSETW::VALUE1 => false,
            LPACSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPACSETW<'a> {
    w: &'a mut W,
}
impl<'a> _LPACSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPACSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACSETW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACSETW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HINTST`"]
pub enum HINTSTW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HINTSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HINTSTW::VALUE1 => false,
            HINTSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HINTSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HINTSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HINTSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTSTW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTSTW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HINTCLR`"]
pub enum HINTCLRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HINTCLRW::VALUE1 => false,
            HINTCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HINTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTCLRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTCLRW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HINTSET`"]
pub enum HINTSETW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HINTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HINTSETW::VALUE1 => false,
            HINTSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _HINTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HINTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTSETW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTSETW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDCLR`"]
pub enum HDCLRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HDCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCLRW::VALUE1 => false,
            HDCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCLRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCLRW::VALUE2)
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
#[doc = "Values that can be written to the field `HDSET`"]
pub enum HDSETW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDSETW::VALUE1 => false,
            HDSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _HDSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDSETW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDSETW::VALUE2)
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
#[doc = "Values that can be written to the field `HDCR`"]
pub enum HDCRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl HDCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCRW::VALUE1 => false,
            HDCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCRW::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSICTRL`"]
pub enum OSCSICTRLW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl OSCSICTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSICTRLW::VALUE1 => false,
            OSCSICTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSICTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSICTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSICTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCSICTRLW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCSICTRLW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCULCTRL`"]
pub enum OSCULCTRLW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl OSCULCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCULCTRLW::VALUE1 => false,
            OSCULCTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCULCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCULCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCULCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCULCTRLW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCULCTRLW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_CTR`"]
pub enum RTC_CTRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RTC_CTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_CTRW::VALUE1 => false,
            RTC_CTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_CTRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_CTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_CTRW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_CTRW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM0`"]
pub enum RTC_ATIM0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RTC_ATIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM0W::VALUE1 => false,
            RTC_ATIM0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM1`"]
pub enum RTC_ATIM1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RTC_ATIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM1W::VALUE1 => false,
            RTC_ATIM1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM0`"]
pub enum RTC_TIM0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RTC_TIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM0W::VALUE1 => false,
            RTC_TIM0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM0W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM0W::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM1`"]
pub enum RTC_TIM1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RTC_TIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM1W::VALUE1 => false,
            RTC_TIM1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM1W::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM1W::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMX`"]
pub enum RMXW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear the status bit"]
    VALUE2,
}
impl RMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMXW::VALUE1 => false,
            RMXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMXW<'a> {
    w: &'a mut W,
}
impl<'a> _RMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMXW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMXW::VALUE2)
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline]
    pub fn prwarn(&mut self) -> _PRWARNW {
        _PRWARNW { w: self }
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline]
    pub fn pi(&mut self) -> _PIW {
        _PIW { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline]
    pub fn ai(&mut self) -> _AIW {
        _AIW { w: self }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline]
    pub fn dlrovr(&mut self) -> _DLROVRW {
        _DLROVRW { w: self }
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpaccr(&mut self) -> _LPACCRW {
        _LPACCRW { w: self }
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpacth0(&mut self) -> _LPACTH0W {
        _LPACTH0W { w: self }
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpacth1(&mut self) -> _LPACTH1W {
        _LPACTH1W { w: self }
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpacst(&mut self) -> _LPACSTW {
        _LPACSTW { w: self }
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpacclr(&mut self) -> _LPACCLRW {
        _LPACCLRW { w: self }
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn lpacset(&mut self) -> _LPACSETW {
        _LPACSETW { w: self }
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn hintst(&mut self) -> _HINTSTW {
        _HINTSTW { w: self }
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn hintclr(&mut self) -> _HINTCLRW {
        _HINTCLRW { w: self }
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Clear"]
    #[inline]
    pub fn hintset(&mut self) -> _HINTSETW {
        _HINTSETW { w: self }
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline]
    pub fn hdclr(&mut self) -> _HDCLRW {
        _HDCLRW { w: self }
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline]
    pub fn hdset(&mut self) -> _HDSETW {
        _HDSETW { w: self }
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline]
    pub fn hdcr(&mut self) -> _HDCRW {
        _HDCRW { w: self }
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline]
    pub fn oscsictrl(&mut self) -> _OSCSICTRLW {
        _OSCSICTRLW { w: self }
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline]
    pub fn osculctrl(&mut self) -> _OSCULCTRLW {
        _OSCULCTRLW { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_ctr(&mut self) -> _RTC_CTRW {
        _RTC_CTRW { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_atim0(&mut self) -> _RTC_ATIM0W {
        _RTC_ATIM0W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_atim1(&mut self) -> _RTC_ATIM1W {
        _RTC_ATIM1W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_tim0(&mut self) -> _RTC_TIM0W {
        _RTC_TIM0W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_tim1(&mut self) -> _RTC_TIM1W {
        _RTC_TIM1W { w: self }
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline]
    pub fn rmx(&mut self) -> _RMXW {
        _RMXW { w: self }
    }
}
