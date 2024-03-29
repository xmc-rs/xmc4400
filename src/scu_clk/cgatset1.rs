#[doc = "Register `CGATSET1` writer"]
pub type W = crate::W<Cgatset1Spec>;
#[doc = "CCU43 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu43 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Ccu43> for bool {
    #[inline(always)]
    fn from(variant: Ccu43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43` writer - CCU43 Gating Set"]
pub type Ccu43W<'a, REG> = crate::BitWriter<'a, REG, Ccu43>;
impl<'a, REG> Ccu43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu43::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu43::Value2)
    }
}
#[doc = "LEDTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledtscu0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Ledtscu0> for bool {
    #[inline(always)]
    fn from(variant: Ledtscu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` writer - LEDTS Gating Set"]
pub type Ledtscu0W<'a, REG> = crate::BitWriter<'a, REG, Ledtscu0>;
impl<'a, REG> Ledtscu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0::Value2)
    }
}
#[doc = "MultiCAN Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcan0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Mcan0> for bool {
    #[inline(always)]
    fn from(variant: Mcan0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` writer - MultiCAN Gating Set"]
pub type Mcan0W<'a, REG> = crate::BitWriter<'a, REG, Mcan0>;
impl<'a, REG> Mcan0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0::Value2)
    }
}
#[doc = "DAC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Dac> for bool {
    #[inline(always)]
    fn from(variant: Dac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` writer - DAC Gating Set"]
pub type DacW<'a, REG> = crate::BitWriter<'a, REG, Dac>;
impl<'a, REG> DacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dac::Value2)
    }
}
#[doc = "USIC1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Usic1> for bool {
    #[inline(always)]
    fn from(variant: Usic1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` writer - USIC1 Gating Set"]
pub type Usic1W<'a, REG> = crate::BitWriter<'a, REG, Usic1>;
impl<'a, REG> Usic1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1::Value2)
    }
}
#[doc = "PORTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pports {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable gating"]
    Value2 = 1,
}
impl From<Pports> for bool {
    #[inline(always)]
    fn from(variant: Pports) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` writer - PORTS Gating Set"]
pub type PportsW<'a, REG> = crate::BitWriter<'a, REG, Pports>;
impl<'a, REG> PportsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pports::Value1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pports::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43(&mut self) -> Ccu43W<Cgatset1Spec> {
        Ccu43W::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0(&mut self) -> Ledtscu0W<Cgatset1Spec> {
        Ledtscu0W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0(&mut self) -> Mcan0W<Cgatset1Spec> {
        Mcan0W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DacW<Cgatset1Spec> {
        DacW::new(self, 5)
    }
    #[doc = "Bit 7 - USIC1 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usic1(&mut self) -> Usic1W<Cgatset1Spec> {
        Usic1W::new(self, 7)
    }
    #[doc = "Bit 9 - PORTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn pports(&mut self) -> PportsW<Cgatset1Spec> {
        PportsW::new(self, 9)
    }
}
#[doc = "Peripheral 1 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatset1Spec;
impl crate::RegisterSpec for Cgatset1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatset1::W`](W) writer structure"]
impl crate::Writable for Cgatset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATSET1 to value 0"]
impl crate::Resettable for Cgatset1Spec {
    const RESET_VALUE: u32 = 0;
}
