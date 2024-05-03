#[doc = "Register `PRSET1` writer"]
pub type W = crate::W<PRSET1_SPEC>;
#[doc = "CCU43 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU43RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU43RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43RS` writer - CCU43 Reset Assert"]
pub type CCU43RS_W<'a, REG> = crate::BitWriter<'a, REG, CCU43RS_A>;
impl<'a, REG> CCU43RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_A::VALUE2)
    }
}
#[doc = "LEDTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<LEDTSCU0RS_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Assert"]
pub type LEDTSCU0RS_W<'a, REG> = crate::BitWriter<'a, REG, LEDTSCU0RS_A>;
impl<'a, REG> LEDTSCU0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_A::VALUE2)
    }
}
#[doc = "MultiCAN Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<MCAN0RS_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Assert"]
pub type MCAN0RS_W<'a, REG> = crate::BitWriter<'a, REG, MCAN0RS_A>;
impl<'a, REG> MCAN0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_A::VALUE2)
    }
}
#[doc = "DAC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DACRS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Assert"]
pub type DACRS_W<'a, REG> = crate::BitWriter<'a, REG, DACRS_A>;
impl<'a, REG> DACRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_A::VALUE2)
    }
}
#[doc = "USIC1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USIC1RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Assert"]
pub type USIC1RS_W<'a, REG> = crate::BitWriter<'a, REG, USIC1RS_A>;
impl<'a, REG> USIC1RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_A::VALUE2)
    }
}
#[doc = "PORTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<PPORTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Assert"]
pub type PPORTSRS_W<'a, REG> = crate::BitWriter<'a, REG, PPORTSRS_A>;
impl<'a, REG> PPORTSRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43rs(&mut self) -> CCU43RS_W<PRSET1_SPEC> {
        CCU43RS_W::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W<PRSET1_SPEC> {
        LEDTSCU0RS_W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W<PRSET1_SPEC> {
        MCAN0RS_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dacrs(&mut self) -> DACRS_W<PRSET1_SPEC> {
        DACRS_W::new(self, 5)
    }
    #[doc = "Bit 7 - USIC1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic1rs(&mut self) -> USIC1RS_W<PRSET1_SPEC> {
        USIC1RS_W::new(self, 7)
    }
    #[doc = "Bit 9 - PORTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn pportsrs(&mut self) -> PPORTSRS_W<PRSET1_SPEC> {
        PPORTSRS_W::new(self, 9)
    }
}
#[doc = "RCU Peripheral 1 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET1_SPEC;
impl crate::RegisterSpec for PRSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset1::W`](W) writer structure"]
impl crate::Writable for PRSET1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSET1 to value 0"]
impl crate::Resettable for PRSET1_SPEC {
    const RESET_VALUE: u32 = 0;
}
