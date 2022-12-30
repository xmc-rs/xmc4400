#[doc = "Register `EXTCLKCR` reader"]
pub struct R(crate::R<EXTCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCLKCR` writer"]
pub struct W(crate::W<EXTCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXTCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECKSEL` reader - External Clock Selection Value"]
pub type ECKSEL_R = crate::FieldReader<u8, ECKSEL_A>;
#[doc = "External Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECKSEL_A {
    #[doc = "0: fSYS clock"]
    VALUE1 = 0,
    #[doc = "2: fUSB clock"]
    VALUE3 = 2,
    #[doc = "3: fPLL clock divided according to ECKDIV bit field configuration"]
    VALUE4 = 3,
    #[doc = "4: fSTDBY clock"]
    VALUE5 = 4,
}
impl From<ECKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ECKSEL_A) -> Self {
        variant as _
    }
}
impl ECKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECKSEL_A> {
        match self.bits {
            0 => Some(ECKSEL_A::VALUE1),
            2 => Some(ECKSEL_A::VALUE3),
            3 => Some(ECKSEL_A::VALUE4),
            4 => Some(ECKSEL_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECKSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ECKSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ECKSEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ECKSEL_A::VALUE5
    }
}
#[doc = "Field `ECKSEL` writer - External Clock Selection Value"]
pub type ECKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTCLKCR_SPEC, u8, ECKSEL_A, 3, O>;
impl<'a, const O: u8> ECKSEL_W<'a, O> {
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE1)
    }
    #[doc = "fUSB clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE3)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE4)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE5)
    }
}
#[doc = "Field `ECKDIV` reader - External Clock Divider Value"]
pub type ECKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ECKDIV` writer - External Clock Divider Value"]
pub type ECKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTCLKCR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:2 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&self) -> ECKSEL_R {
        ECKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&self) -> ECKDIV_R {
        ECKDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecksel(&mut self) -> ECKSEL_W<0> {
        ECKSEL_W::new(self)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn eckdiv(&mut self) -> ECKDIV_W<16> {
        ECKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclkcr](index.html) module"]
pub struct EXTCLKCR_SPEC;
impl crate::RegisterSpec for EXTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extclkcr::R](R) reader structure"]
impl crate::Readable for EXTCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extclkcr::W](W) writer structure"]
impl crate::Writable for EXTCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTCLKCR to value 0"]
impl crate::Resettable for EXTCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
