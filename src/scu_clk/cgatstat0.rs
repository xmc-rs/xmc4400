#[doc = "Register `CGATSTAT0` reader"]
pub struct R(crate::R<CGATSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGATSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGATSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGATSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` reader - VADC Gating Status"]
pub struct VADC_R(crate::FieldReader<bool, VADC_A>);
impl VADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::VALUE1,
            true => VADC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VADC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VADC_A::VALUE2
    }
}
impl core::ops::Deref for VADC_R {
    type Target = crate::FieldReader<bool, VADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DSD Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSD_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<DSD_A> for bool {
    #[inline(always)]
    fn from(variant: DSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSD` reader - DSD Gating Status"]
pub struct DSD_R(crate::FieldReader<bool, DSD_A>);
impl DSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSD_A {
        match self.bits {
            false => DSD_A::VALUE1,
            true => DSD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DSD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DSD_A::VALUE2
    }
}
impl core::ops::Deref for DSD_R {
    type Target = crate::FieldReader<bool, DSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` reader - CCU40 Gating Status"]
pub struct CCU40_R(crate::FieldReader<bool, CCU40_A>);
impl CCU40_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU40_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::VALUE1,
            true => CCU40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCU40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCU40_A::VALUE2
    }
}
impl core::ops::Deref for CCU40_R {
    type Target = crate::FieldReader<bool, CCU40_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU41_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` reader - CCU41 Gating Status"]
pub struct CCU41_R(crate::FieldReader<bool, CCU41_A>);
impl CCU41_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU41_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41_A {
        match self.bits {
            false => CCU41_A::VALUE1,
            true => CCU41_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCU41_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCU41_A::VALUE2
    }
}
impl core::ops::Deref for CCU41_R {
    type Target = crate::FieldReader<bool, CCU41_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU42 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU42_A> for bool {
    #[inline(always)]
    fn from(variant: CCU42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42` reader - CCU42 Gating Status"]
pub struct CCU42_R(crate::FieldReader<bool, CCU42_A>);
impl CCU42_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU42_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU42_A {
        match self.bits {
            false => CCU42_A::VALUE1,
            true => CCU42_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCU42_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCU42_A::VALUE2
    }
}
impl core::ops::Deref for CCU42_R {
    type Target = crate::FieldReader<bool, CCU42_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU80_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` reader - CCU80 Gating Status"]
pub struct CCU80_R(crate::FieldReader<bool, CCU80_A>);
impl CCU80_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU80_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80_A {
        match self.bits {
            false => CCU80_A::VALUE1,
            true => CCU80_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCU80_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCU80_A::VALUE2
    }
}
impl core::ops::Deref for CCU80_R {
    type Target = crate::FieldReader<bool, CCU80_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CCU81 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU81_A> for bool {
    #[inline(always)]
    fn from(variant: CCU81_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81` reader - CCU81 Gating Status"]
pub struct CCU81_R(crate::FieldReader<bool, CCU81_A>);
impl CCU81_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCU81_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU81_A {
        match self.bits {
            false => CCU81_A::VALUE1,
            true => CCU81_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCU81_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCU81_A::VALUE2
    }
}
impl core::ops::Deref for CCU81_R {
    type Target = crate::FieldReader<bool, CCU81_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<POSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` reader - POSIF0 Gating Status"]
pub struct POSIF0_R(crate::FieldReader<bool, POSIF0_A>);
impl POSIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        POSIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0_A {
        match self.bits {
            false => POSIF0_A::VALUE1,
            true => POSIF0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == POSIF0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == POSIF0_A::VALUE2
    }
}
impl core::ops::Deref for POSIF0_R {
    type Target = crate::FieldReader<bool, POSIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "POSIF1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<POSIF1_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1` reader - POSIF1 Gating Status"]
pub struct POSIF1_R(crate::FieldReader<bool, POSIF1_A>);
impl POSIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        POSIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF1_A {
        match self.bits {
            false => POSIF1_A::VALUE1,
            true => POSIF1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == POSIF1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == POSIF1_A::VALUE2
    }
}
impl core::ops::Deref for POSIF1_R {
    type Target = crate::FieldReader<bool, POSIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` reader - USIC0 Gating Status"]
pub struct USIC0_R(crate::FieldReader<bool, USIC0_A>);
impl USIC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::VALUE1,
            true => USIC0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == USIC0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == USIC0_A::VALUE2
    }
}
impl core::ops::Deref for USIC0_R {
    type Target = crate::FieldReader<bool, USIC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<ERU1_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` reader - ERU1 Gating Status"]
pub struct ERU1_R(crate::FieldReader<bool, ERU1_A>);
impl ERU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1_A {
        match self.bits {
            false => ERU1_A::VALUE1,
            true => ERU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ERU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERU1_A::VALUE2
    }
}
impl core::ops::Deref for ERU1_R {
    type Target = crate::FieldReader<bool, ERU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRPWM0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRPWM0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<HRPWM0_A> for bool {
    #[inline(always)]
    fn from(variant: HRPWM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRPWM0` reader - HRPWM0 Gating Status"]
pub struct HRPWM0_R(crate::FieldReader<bool, HRPWM0_A>);
impl HRPWM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRPWM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPWM0_A {
        match self.bits {
            false => HRPWM0_A::VALUE1,
            true => HRPWM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HRPWM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HRPWM0_A::VALUE2
    }
}
impl core::ops::Deref for HRPWM0_R {
    type Target = crate::FieldReader<bool, HRPWM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSD Gating Status"]
    #[inline(always)]
    pub fn dsd(&self) -> DSD_R {
        DSD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> CCU41_R {
        CCU41_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CCU42 Gating Status"]
    #[inline(always)]
    pub fn ccu42(&self) -> CCU42_R {
        CCU42_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> CCU80_R {
        CCU80_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCU81 Gating Status"]
    #[inline(always)]
    pub fn ccu81(&self) -> CCU81_R {
        CCU81_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> POSIF0_R {
        POSIF0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - POSIF1 Gating Status"]
    #[inline(always)]
    pub fn posif1(&self) -> POSIF1_R {
        POSIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> ERU1_R {
        ERU1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HRPWM0 Gating Status"]
    #[inline(always)]
    pub fn hrpwm0(&self) -> HRPWM0_R {
        HRPWM0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Peripheral 0 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat0](index.html) module"]
pub struct CGATSTAT0_SPEC;
impl crate::RegisterSpec for CGATSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgatstat0::R](R) reader structure"]
impl crate::Readable for CGATSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CGATSTAT0 to value 0"]
impl crate::Resettable for CGATSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
