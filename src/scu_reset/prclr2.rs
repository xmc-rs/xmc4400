#[doc = "Register `PRCLR2` writer"]
pub type W = crate::W<Prclr2Spec>;
#[doc = "WDT Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Wdtrs> for bool {
    #[inline(always)]
    fn from(variant: Wdtrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Clear"]
pub type WdtrsW<'a, REG> = crate::BitWriter<'a, REG, Wdtrs>;
impl<'a, REG> WdtrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrs::Value2)
    }
}
#[doc = "ETH0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Eth0rs> for bool {
    #[inline(always)]
    fn from(variant: Eth0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Clear"]
pub type Eth0rsW<'a, REG> = crate::BitWriter<'a, REG, Eth0rs>;
impl<'a, REG> Eth0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0rs::Value2)
    }
}
#[doc = "DMA0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Dma0rs> for bool {
    #[inline(always)]
    fn from(variant: Dma0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Clear"]
pub type Dma0rsW<'a, REG> = crate::BitWriter<'a, REG, Dma0rs>;
impl<'a, REG> Dma0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rs::Value2)
    }
}
#[doc = "FCE Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcers {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Fcers> for bool {
    #[inline(always)]
    fn from(variant: Fcers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Clear"]
pub type FcersW<'a, REG> = crate::BitWriter<'a, REG, Fcers>;
impl<'a, REG> FcersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcers::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcers::Value2)
    }
}
#[doc = "USB Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Usbrs> for bool {
    #[inline(always)]
    fn from(variant: Usbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Clear"]
pub type UsbrsW<'a, REG> = crate::BitWriter<'a, REG, Usbrs>;
impl<'a, REG> UsbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrs::Value2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WdtrsW<Prclr2Spec> {
        WdtrsW::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0rs(&mut self) -> Eth0rsW<Prclr2Spec> {
        Eth0rsW::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rs(&mut self) -> Dma0rsW<Prclr2Spec> {
        Dma0rsW::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fcers(&mut self) -> FcersW<Prclr2Spec> {
        FcersW::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbrs(&mut self) -> UsbrsW<Prclr2Spec> {
        UsbrsW::new(self, 7)
    }
}
#[doc = "RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prclr2Spec;
impl crate::RegisterSpec for Prclr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr2::W`](W) writer structure"]
impl crate::Writable for Prclr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR2 to value 0"]
impl crate::Resettable for Prclr2Spec {
    const RESET_VALUE: u32 = 0;
}
