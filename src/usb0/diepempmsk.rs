#[doc = "Register `DIEPEMPMSK` reader"]
pub struct R(crate::R<DIEPEMPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPEMPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPEMPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPEMPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPEMPMSK` writer"]
pub struct W(crate::W<DIEPEMPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPEMPMSK_SPEC>;
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
impl From<crate::W<DIEPEMPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPEMPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `InEpTxfEmpMsk` reader - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type IN_EP_TXF_EMP_MSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `InEpTxfEmpMsk` writer - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type IN_EP_TXF_EMP_MSK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPEMPMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_txf_emp_msk(&self) -> IN_EP_TXF_EMP_MSK_R {
        IN_EP_TXF_EMP_MSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn in_ep_txf_emp_msk(&mut self) -> IN_EP_TXF_EMP_MSK_W<0> {
        IN_EP_TXF_EMP_MSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepempmsk](index.html) module"]
pub struct DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepempmsk::R](R) reader structure"]
impl crate::Readable for DIEPEMPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](W) writer structure"]
impl crate::Writable for DIEPEMPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DIEPEMPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
