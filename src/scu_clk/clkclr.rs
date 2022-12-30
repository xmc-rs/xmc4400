#[doc = "Register `CLKCLR` writer"]
pub struct W(crate::W<CLKCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCLR_SPEC>;
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
impl From<crate::W<CLKCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<USBCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCDI` writer - USB Clock Disable"]
pub type USBCDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCLR_SPEC, USBCDI_AW, O>;
impl<'a, const O: u8> USBCDI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCDI_AW::VALUE2)
    }
}
#[doc = "Ethernet Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<ETH0CDI_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CDI` writer - Ethernet Clock Disable"]
pub type ETH0CDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCLR_SPEC, ETH0CDI_AW, O>;
impl<'a, const O: u8> ETH0CDI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CDI_AW::VALUE2)
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<CCUCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCDI` writer - CCU Clock Disable"]
pub type CCUCDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCLR_SPEC, CCUCDI_AW, O>;
impl<'a, const O: u8> CCUCDI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCDI_AW::VALUE2)
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<WDTCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCDI` writer - WDT Clock Disable"]
pub type WDTCDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCLR_SPEC, WDTCDI_AW, O>;
impl<'a, const O: u8> WDTCDI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCDI_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcdi(&mut self) -> USBCDI_W<0> {
        USBCDI_W::new(self)
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cdi(&mut self) -> ETH0CDI_W<2> {
        ETH0CDI_W::new(self)
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucdi(&mut self) -> CCUCDI_W<4> {
        CCUCDI_W::new(self)
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcdi(&mut self) -> WDTCDI_W<5> {
        WDTCDI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkclr](index.html) module"]
pub struct CLKCLR_SPEC;
impl crate::RegisterSpec for CLKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clkclr::W](W) writer structure"]
impl crate::Writable for CLKCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for CLKCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
