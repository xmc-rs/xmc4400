#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PL {
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
#[doc = "Possible values of the field `PSL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL0R {
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    VALUE1,
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    VALUE2,
}
impl PSL0R {
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
            PSL0R::VALUE1 => false,
            PSL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL0R {
        match value {
            false => PSL0R::VALUE1,
            true => PSL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL0R::VALUE2
    }
}
#[doc = "Possible values of the field `PSL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL1R {
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    VALUE1,
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    VALUE2,
}
impl PSL1R {
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
            PSL1R::VALUE1 => false,
            PSL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSL1R {
        match value {
            false => PSL1R::VALUE1,
            true => PSL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSL1R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PSL0`"]
pub enum PSL0W {
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    VALUE1,
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    VALUE2,
}
impl PSL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL0W::VALUE1 => false,
            PSL0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL0W::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL0W::VALUE2)
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
#[doc = "Values that can be written to the field `PSL1`"]
pub enum PSL1W {
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    VALUE1,
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    VALUE2,
}
impl PSL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSL1W::VALUE1 => false,
            PSL1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PSL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL1W::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL1W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline]
    pub fn psl0(&self) -> PSL0R {
        PSL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline]
    pub fn psl1(&self) -> PSL1R {
        PSL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline]
    pub fn psl0(&mut self) -> _PSL0W {
        _PSL0W { w: self }
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline]
    pub fn psl1(&mut self) -> _PSL1W {
        _PSL1W { w: self }
    }
}
