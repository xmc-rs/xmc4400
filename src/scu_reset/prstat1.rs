#[doc = "Register `PRSTAT1` reader"]
pub type R = crate::R<PRSTAT1_SPEC>;
#[doc = "CCU43 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU43RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU43RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43RS` reader - CCU43 Reset Status"]
pub type CCU43RS_R = crate::BitReader<CCU43RS_A>;
impl CCU43RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU43RS_A {
        match self.bits {
            false => CCU43RS_A::VALUE1,
            true => CCU43RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU43RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU43RS_A::VALUE2
    }
}
#[doc = "LEDTS Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<LEDTSCU0RS_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` reader - LEDTS Reset Status"]
pub type LEDTSCU0RS_R = crate::BitReader<LEDTSCU0RS_A>;
impl LEDTSCU0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEDTSCU0RS_A {
        match self.bits {
            false => LEDTSCU0RS_A::VALUE1,
            true => LEDTSCU0RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0RS_A::VALUE2
    }
}
#[doc = "MultiCAN Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<MCAN0RS_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` reader - MultiCAN Reset Status"]
pub type MCAN0RS_R = crate::BitReader<MCAN0RS_A>;
impl MCAN0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCAN0RS_A {
        match self.bits {
            false => MCAN0RS_A::VALUE1,
            true => MCAN0RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0RS_A::VALUE2
    }
}
#[doc = "DAC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<DACRS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` reader - DAC Reset Status"]
pub type DACRS_R = crate::BitReader<DACRS_A>;
impl DACRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DACRS_A {
        match self.bits {
            false => DACRS_A::VALUE1,
            true => DACRS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DACRS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DACRS_A::VALUE2
    }
}
#[doc = "USIC1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<USIC1RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` reader - USIC1 Reset Status"]
pub type USIC1RS_R = crate::BitReader<USIC1RS_A>;
impl USIC1RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC1RS_A {
        match self.bits {
            false => USIC1RS_A::VALUE1,
            true => USIC1RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1RS_A::VALUE2
    }
}
#[doc = "PORTS Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<PPORTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` reader - PORTS Reset Status"]
pub type PPORTSRS_R = crate::BitReader<PPORTSRS_A>;
impl PPORTSRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPORTSRS_A {
        match self.bits {
            false => PPORTSRS_A::VALUE1,
            true => PPORTSRS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPORTSRS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPORTSRS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - CCU43 Reset Status"]
    #[inline(always)]
    pub fn ccu43rs(&self) -> CCU43RS_R {
        CCU43RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline(always)]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RS_R {
        LEDTSCU0RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline(always)]
    pub fn mcan0rs(&self) -> MCAN0RS_R {
        MCAN0RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline(always)]
    pub fn dacrs(&self) -> DACRS_R {
        DACRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline(always)]
    pub fn usic1rs(&self) -> USIC1RS_R {
        USIC1RS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline(always)]
    pub fn pportsrs(&self) -> PPORTSRS_R {
        PPORTSRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 1 Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`prstat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSTAT1_SPEC;
impl crate::RegisterSpec for PRSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat1::R`](R) reader structure"]
impl crate::Readable for PRSTAT1_SPEC {}
#[doc = "`reset()` method sets PRSTAT1 to value 0xb9"]
impl crate::Resettable for PRSTAT1_SPEC {
    const RESET_VALUE: u32 = 0xb9;
}
