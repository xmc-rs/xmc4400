#[doc = "Register `LPACTH1` reader"]
pub struct R(crate::R<LPACTH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPACTH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPACTH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPACTH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPACTH1` writer"]
pub struct W(crate::W<LPACTH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPACTH1_SPEC>;
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
impl From<crate::W<LPACTH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPACTH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHIBIO0LO` reader - Analog HIB_IO_0 Lower Threshold Value"]
pub struct AHIBIO0LO_R(crate::FieldReader<u8, u8>);
impl AHIBIO0LO_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHIBIO0LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHIBIO0LO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHIBIO0LO` writer - Analog HIB_IO_0 Lower Threshold Value"]
pub struct AHIBIO0LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `AHIBIO0HI` reader - Analog HIB_IO_0 Upper Threshold Value"]
pub struct AHIBIO0HI_R(crate::FieldReader<u8, u8>);
impl AHIBIO0HI_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHIBIO0HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHIBIO0HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHIBIO0HI` writer - Analog HIB_IO_0 Upper Threshold Value"]
pub struct AHIBIO0HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `AHIBIO1LO` reader - Analog HIB_IO_1 Lower Threshold Value"]
pub struct AHIBIO1LO_R(crate::FieldReader<u8, u8>);
impl AHIBIO1LO_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHIBIO1LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHIBIO1LO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHIBIO1LO` writer - Analog HIB_IO_1 Lower Threshold Value"]
pub struct AHIBIO1LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO1LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `AHIBIO1HI` reader - Analog HIB_IO_1 Upper Threshold Value"]
pub struct AHIBIO1HI_R(crate::FieldReader<u8, u8>);
impl AHIBIO1HI_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHIBIO1HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHIBIO1HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHIBIO1HI` writer - Analog HIB_IO_1 Upper Threshold Value"]
pub struct AHIBIO1HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO1HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> AHIBIO0LO_R {
        AHIBIO0LO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> AHIBIO0HI_R {
        AHIBIO0HI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Analog HIB_IO_1 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio1lo(&self) -> AHIBIO1LO_R {
        AHIBIO1LO_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Analog HIB_IO_1 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio1hi(&self) -> AHIBIO1HI_R {
        AHIBIO1HI_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&mut self) -> AHIBIO0LO_W {
        AHIBIO0LO_W { w: self }
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&mut self) -> AHIBIO0HI_W {
        AHIBIO0HI_W { w: self }
    }
    #[doc = "Bits 16:21 - Analog HIB_IO_1 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio1lo(&mut self) -> AHIBIO1LO_W {
        AHIBIO1LO_W { w: self }
    }
    #[doc = "Bits 24:29 - Analog HIB_IO_1 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio1hi(&mut self) -> AHIBIO1HI_W {
        AHIBIO1HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPAC Threshold Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacth1](index.html) module"]
pub struct LPACTH1_SPEC;
impl crate::RegisterSpec for LPACTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpacth1::R](R) reader structure"]
impl crate::Readable for LPACTH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpacth1::W](W) writer structure"]
impl crate::Writable for LPACTH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPACTH1 to value 0"]
impl crate::Resettable for LPACTH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
