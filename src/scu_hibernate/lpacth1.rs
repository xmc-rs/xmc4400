#[doc = "Register `LPACTH1` reader"]
pub type R = crate::R<Lpacth1Spec>;
#[doc = "Register `LPACTH1` writer"]
pub type W = crate::W<Lpacth1Spec>;
#[doc = "Field `AHIBIO0LO` reader - Analog HIB_IO_0 Lower Threshold Value"]
pub type Ahibio0loR = crate::FieldReader;
#[doc = "Field `AHIBIO0LO` writer - Analog HIB_IO_0 Lower Threshold Value"]
pub type Ahibio0loW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHIBIO0HI` reader - Analog HIB_IO_0 Upper Threshold Value"]
pub type Ahibio0hiR = crate::FieldReader;
#[doc = "Field `AHIBIO0HI` writer - Analog HIB_IO_0 Upper Threshold Value"]
pub type Ahibio0hiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHIBIO1LO` reader - Analog HIB_IO_1 Lower Threshold Value"]
pub type Ahibio1loR = crate::FieldReader;
#[doc = "Field `AHIBIO1LO` writer - Analog HIB_IO_1 Lower Threshold Value"]
pub type Ahibio1loW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHIBIO1HI` reader - Analog HIB_IO_1 Upper Threshold Value"]
pub type Ahibio1hiR = crate::FieldReader;
#[doc = "Field `AHIBIO1HI` writer - Analog HIB_IO_1 Upper Threshold Value"]
pub type Ahibio1hiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> Ahibio0loR {
        Ahibio0loR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> Ahibio0hiR {
        Ahibio0hiR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Analog HIB_IO_1 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio1lo(&self) -> Ahibio1loR {
        Ahibio1loR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Analog HIB_IO_1 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio1hi(&self) -> Ahibio1hiR {
        Ahibio1hiR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0lo(&mut self) -> Ahibio0loW<Lpacth1Spec> {
        Ahibio0loW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0hi(&mut self) -> Ahibio0hiW<Lpacth1Spec> {
        Ahibio0hiW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Analog HIB_IO_1 Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1lo(&mut self) -> Ahibio1loW<Lpacth1Spec> {
        Ahibio1loW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Analog HIB_IO_1 Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1hi(&mut self) -> Ahibio1hiW<Lpacth1Spec> {
        Ahibio1hiW::new(self, 24)
    }
}
#[doc = "LPAC Threshold Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpacth1Spec;
impl crate::RegisterSpec for Lpacth1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacth1::R`](R) reader structure"]
impl crate::Readable for Lpacth1Spec {}
#[doc = "`write(|w| ..)` method takes [`lpacth1::W`](W) writer structure"]
impl crate::Writable for Lpacth1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACTH1 to value 0"]
impl crate::Resettable for Lpacth1Spec {
    const RESET_VALUE: u32 = 0;
}
