#[doc = "Register `PRSET2` writer"]
pub type W = crate::W<PRSET2_SPEC>;
#[doc = "WDT Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<WDTRS_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Assert"]
pub type WDTRS_W<'a, REG> = crate::BitWriter<'a, REG, WDTRS_AW>;
impl<'a, REG> WDTRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::VALUE2)
    }
}
#[doc = "ETH0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<ETH0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Assert"]
pub type ETH0RS_W<'a, REG> = crate::BitWriter<'a, REG, ETH0RS_AW>;
impl<'a, REG> ETH0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::VALUE2)
    }
}
#[doc = "DMA0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DMA0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Assert"]
pub type DMA0RS_W<'a, REG> = crate::BitWriter<'a, REG, DMA0RS_AW>;
impl<'a, REG> DMA0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::VALUE2)
    }
}
#[doc = "FCE Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<FCERS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCERS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Assert"]
pub type FCERS_W<'a, REG> = crate::BitWriter<'a, REG, FCERS_AW>;
impl<'a, REG> FCERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::VALUE2)
    }
}
#[doc = "USB Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Assert"]
pub type USBRS_W<'a, REG> = crate::BitWriter<'a, REG, USBRS_AW>;
impl<'a, REG> USBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WDTRS_W<PRSET2_SPEC> {
        WDTRS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eth0rs(&mut self) -> ETH0RS_W<PRSET2_SPEC> {
        ETH0RS_W::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rs(&mut self) -> DMA0RS_W<PRSET2_SPEC> {
        DMA0RS_W::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn fcers(&mut self) -> FCERS_W<PRSET2_SPEC> {
        FCERS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usbrs(&mut self) -> USBRS_W<PRSET2_SPEC> {
        USBRS_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCU Peripheral 2 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET2_SPEC;
impl crate::RegisterSpec for PRSET2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset2::W`](W) writer structure"]
impl crate::Writable for PRSET2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET2 to value 0"]
impl crate::Resettable for PRSET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
