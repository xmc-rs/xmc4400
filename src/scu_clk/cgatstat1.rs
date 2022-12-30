#[doc = "Register `CGATSTAT1` reader"]
pub struct R(crate::R<CGATSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGATSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGATSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGATSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCU43` reader - CCU43 Gating Status"]
pub type CCU43_R = crate::BitReader<CCU43_A>;
#[doc = "CCU43 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU43_A> for bool {
    #[inline(always)]
    fn from(variant: CCU43_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU43_A {
        match self.bits {
            false => CCU43_A::VALUE1,
            true => CCU43_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU43_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU43_A::VALUE2
    }
}
#[doc = "Field `LEDTSCU0` reader - LEDTS Gating Status"]
pub type LEDTSCU0_R = crate::BitReader<LEDTSCU0_A>;
#[doc = "LEDTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<LEDTSCU0_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_A) -> Self {
        variant as u8 != 0
    }
}
impl LEDTSCU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDTSCU0_A {
        match self.bits {
            false => LEDTSCU0_A::VALUE1,
            true => LEDTSCU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0_A::VALUE2
    }
}
#[doc = "Field `MCAN0` reader - MultiCAN Gating Status"]
pub type MCAN0_R = crate::BitReader<MCAN0_A>;
#[doc = "MultiCAN Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<MCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_A) -> Self {
        variant as u8 != 0
    }
}
impl MCAN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCAN0_A {
        match self.bits {
            false => MCAN0_A::VALUE1,
            true => MCAN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0_A::VALUE2
    }
}
#[doc = "Field `DAC` reader - DAC Gating Status"]
pub type DAC_R = crate::BitReader<DAC_A>;
#[doc = "DAC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<DAC_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_A {
        match self.bits {
            false => DAC_A::VALUE1,
            true => DAC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DAC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DAC_A::VALUE2
    }
}
#[doc = "Field `USIC1` reader - USIC1 Gating Status"]
pub type USIC1_R = crate::BitReader<USIC1_A>;
#[doc = "USIC1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<USIC1_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1_A) -> Self {
        variant as u8 != 0
    }
}
impl USIC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1_A {
        match self.bits {
            false => USIC1_A::VALUE1,
            true => USIC1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1_A::VALUE2
    }
}
#[doc = "Field `PPORTS` reader - PORTS Gating Status"]
pub type PPORTS_R = crate::BitReader<PPORTS_A>;
#[doc = "PORTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTS_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<PPORTS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_A) -> Self {
        variant as u8 != 0
    }
}
impl PPORTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPORTS_A {
        match self.bits {
            false => PPORTS_A::VALUE1,
            true => PPORTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPORTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPORTS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - CCU43 Gating Status"]
    #[inline(always)]
    pub fn ccu43(&self) -> CCU43_R {
        CCU43_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - LEDTS Gating Status"]
    #[inline(always)]
    pub fn ledtscu0(&self) -> LEDTSCU0_R {
        LEDTSCU0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Gating Status"]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Gating Status"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Gating Status"]
    #[inline(always)]
    pub fn usic1(&self) -> USIC1_R {
        USIC1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Gating Status"]
    #[inline(always)]
    pub fn pports(&self) -> PPORTS_R {
        PPORTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Peripheral 1 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat1](index.html) module"]
pub struct CGATSTAT1_SPEC;
impl crate::RegisterSpec for CGATSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgatstat1::R](R) reader structure"]
impl crate::Readable for CGATSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CGATSTAT1 to value 0"]
impl crate::Resettable for CGATSTAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
