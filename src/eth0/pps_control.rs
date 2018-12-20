#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPS_CONTROL {
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
pub struct PPSCTRL_PPSCMDR {
    bits: u8,
}
impl PPSCTRL_PPSCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPSEN0R {
    bits: bool,
}
impl PPSEN0R {
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
pub struct TRGTMODSEL0R {
    bits: u8,
}
impl TRGTMODSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPSCMD1R {
    bits: u8,
}
impl PPSCMD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRGTMODSEL1R {
    bits: u8,
}
impl TRGTMODSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPSCMD2R {
    bits: u8,
}
impl PPSCMD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRGTMODSEL2R {
    bits: u8,
}
impl TRGTMODSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPSCMD3R {
    bits: u8,
}
impl PPSCMD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRGTMODSEL3R {
    bits: u8,
}
impl TRGTMODSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PPSCTRL_PPSCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _PPSCTRL_PPSCMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline]
    pub fn ppsctrl_ppscmd(&self) -> PPSCTRL_PPSCMDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPSCTRL_PPSCMDR { bits }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline]
    pub fn ppsen0(&self) -> PPSEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPSEN0R { bits }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGTMODSEL0R { bits }
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control"]
    #[inline]
    pub fn ppscmd1(&self) -> PPSCMD1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPSCMD1R { bits }
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output"]
    #[inline]
    pub fn trgtmodsel1(&self) -> TRGTMODSEL1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGTMODSEL1R { bits }
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control"]
    #[inline]
    pub fn ppscmd2(&self) -> PPSCMD2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPSCMD2R { bits }
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output"]
    #[inline]
    pub fn trgtmodsel2(&self) -> TRGTMODSEL2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGTMODSEL2R { bits }
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control"]
    #[inline]
    pub fn ppscmd3(&self) -> PPSCMD3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPSCMD3R { bits }
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output"]
    #[inline]
    pub fn trgtmodsel3(&self) -> TRGTMODSEL3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGTMODSEL3R { bits }
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
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline]
    pub fn ppsctrl_ppscmd(&mut self) -> _PPSCTRL_PPSCMDW {
        _PPSCTRL_PPSCMDW { w: self }
    }
}
