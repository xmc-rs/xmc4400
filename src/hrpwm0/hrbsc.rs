#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HRBSC {
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
#[doc = "Possible values of the field `SUSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFGR {
    #[doc = "Suspend is ignored."]
    VALUE1,
    #[doc = "CSGy and HRCy units are halted."]
    VALUE2,
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE3,
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUSCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUSCFGR::VALUE1 => 0,
            SUSCFGR::VALUE2 => 1,
            SUSCFGR::VALUE3 => 2,
            SUSCFGR::VALUE4 => 3,
            SUSCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUSCFGR {
        match value {
            0 => SUSCFGR::VALUE1,
            1 => SUSCFGR::VALUE2,
            2 => SUSCFGR::VALUE3,
            3 => SUSCFGR::VALUE4,
            i => SUSCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFGR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct HRBER {
    bits: bool,
}
impl HRBER {
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
#[doc = "Values that can be written to the field `SUSCFG`"]
pub enum SUSCFGW {
    #[doc = "Suspend is ignored."]
    VALUE1,
    #[doc = "CSGy and HRCy units are halted."]
    VALUE2,
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE3,
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE4,
}
impl SUSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SUSCFGW::VALUE1 => 0,
            SUSCFGW::VALUE2 => 1,
            SUSCFGW::VALUE3 => 2,
            SUSCFGW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Suspend is ignored."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE1)
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE2)
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE3)
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE4)
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
#[doc = r" Proxy"]
pub struct _HRBEW<'a> {
    w: &'a mut W,
}
impl<'a> _HRBEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline]
    pub fn suscfg(&self) -> SUSCFGR {
        SUSCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline]
    pub fn hrbe(&self) -> HRBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HRBER { bits }
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
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline]
    pub fn suscfg(&mut self) -> _SUSCFGW {
        _SUSCFGW { w: self }
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline]
    pub fn hrbe(&mut self) -> _HRBEW {
        _HRBEW { w: self }
    }
}
