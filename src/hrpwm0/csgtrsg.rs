#[doc = "Register `CSGTRSG` reader"]
pub type R = crate::R<CSGTRSG_SPEC>;
#[doc = "DAC0 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D0STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2 = 1,
}
impl From<D0STE_A> for bool {
    #[inline(always)]
    fn from(variant: D0STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D0STE` reader - DAC0 shadow transfer enable"]
pub type D0STE_R = crate::BitReader<D0STE_A>;
impl D0STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D0STE_A {
        match self.bits {
            false => D0STE_A::VALUE1,
            true => D0STE_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D0STE_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D0STE_A::VALUE2
    }
}
#[doc = "CMP0 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW0ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C0I\\[A\\]"]
    VALUE1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C0I\\[B\\]"]
    VALUE2 = 1,
}
impl From<SW0ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW0ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW0ST` reader - CMP0 inverting input connection status"]
pub type SW0ST_R = crate::BitReader<SW0ST_A>;
impl SW0ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW0ST_A {
        match self.bits {
            false => SW0ST_A::VALUE1,
            true => SW0ST_A::VALUE2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C0I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW0ST_A::VALUE1
    }
    #[doc = "Inverting input connected to HRPWMx.C0I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW0ST_A::VALUE2
    }
}
#[doc = "DAC1 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D1STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2 = 1,
}
impl From<D1STE_A> for bool {
    #[inline(always)]
    fn from(variant: D1STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D1STE` reader - DAC1 shadow transfer enable"]
pub type D1STE_R = crate::BitReader<D1STE_A>;
impl D1STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D1STE_A {
        match self.bits {
            false => D1STE_A::VALUE1,
            true => D1STE_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D1STE_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D1STE_A::VALUE2
    }
}
#[doc = "CMP1 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW1ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C1I\\[A\\]"]
    VALUE1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C1I\\[B\\]"]
    VALUE2 = 1,
}
impl From<SW1ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW1ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW1ST` reader - CMP1 inverting input connection status"]
pub type SW1ST_R = crate::BitReader<SW1ST_A>;
impl SW1ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW1ST_A {
        match self.bits {
            false => SW1ST_A::VALUE1,
            true => SW1ST_A::VALUE2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C1I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW1ST_A::VALUE1
    }
    #[doc = "Inverting input connected to HRPWMx.C1I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW1ST_A::VALUE2
    }
}
#[doc = "DAC2 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D2STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2 = 1,
}
impl From<D2STE_A> for bool {
    #[inline(always)]
    fn from(variant: D2STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D2STE` reader - DAC2 shadow transfer enable"]
pub type D2STE_R = crate::BitReader<D2STE_A>;
impl D2STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D2STE_A {
        match self.bits {
            false => D2STE_A::VALUE1,
            true => D2STE_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D2STE_A::VALUE1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D2STE_A::VALUE2
    }
}
#[doc = "CMP2 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW2ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C2I\\[A\\]"]
    VALUE1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C2I\\[B\\]"]
    VALUE2 = 1,
}
impl From<SW2ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW2ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW2ST` reader - CMP2 inverting input connection status"]
pub type SW2ST_R = crate::BitReader<SW2ST_A>;
impl SW2ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW2ST_A {
        match self.bits {
            false => SW2ST_A::VALUE1,
            true => SW2ST_A::VALUE2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C2I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW2ST_A::VALUE1
    }
    #[doc = "Inverting input connected to HRPWMx.C2I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW2ST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 shadow transfer enable"]
    #[inline(always)]
    pub fn d0ste(&self) -> D0STE_R {
        D0STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 inverting input connection status"]
    #[inline(always)]
    pub fn sw0st(&self) -> SW0ST_R {
        SW0ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable"]
    #[inline(always)]
    pub fn d1ste(&self) -> D1STE_R {
        D1STE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP1 inverting input connection status"]
    #[inline(always)]
    pub fn sw1st(&self) -> SW1ST_R {
        SW1ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable"]
    #[inline(always)]
    pub fn d2ste(&self) -> D2STE_R {
        D2STE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP2 inverting input connection status"]
    #[inline(always)]
    pub fn sw2st(&self) -> SW2ST_R {
        SW2ST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Global CSG shadow/switch status\n\nYou can [`read`](crate::Reg::read) this register and get [`csgtrsg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGTRSG_SPEC;
impl crate::RegisterSpec for CSGTRSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgtrsg::R`](R) reader structure"]
impl crate::Readable for CSGTRSG_SPEC {}
#[doc = "`reset()` method sets CSGTRSG to value 0"]
impl crate::Resettable for CSGTRSG_SPEC {
    const RESET_VALUE: u32 = 0;
}
