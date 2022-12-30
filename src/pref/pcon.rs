#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl From<crate::W<PCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub type IBYP_R = crate::BitReader<IBYP_A>;
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    VALUE1 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    VALUE2 = 1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl IBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::VALUE1,
            true => IBYP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IBYP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IBYP_A::VALUE2
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub type IBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, IBYP_A, O>;
impl<'a, const O: u8> IBYP_W<'a, O> {
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IBYP_A::VALUE1)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IBYP_A::VALUE2)
    }
}
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINV_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    VALUE2 = 1,
}
impl From<IINV_AW> for bool {
    #[inline(always)]
    fn from(variant: IINV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub type IINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, IINV_AW, O>;
impl<'a, const O: u8> IINV_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IINV_AW::VALUE1)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IINV_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn ibyp(&mut self) -> IBYP_W<0> {
        IBYP_W::new(self)
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    #[must_use]
    pub fn iinv(&mut self) -> IINV_W<1> {
        IINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefetch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
