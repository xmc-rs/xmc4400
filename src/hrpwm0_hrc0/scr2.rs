#[doc = "Register `SCR2` reader"]
pub type R = crate::R<SCR2_SPEC>;
#[doc = "Register `SCR2` writer"]
pub type W = crate::W<SCR2_SPEC>;
#[doc = "Field `SCR2` reader - High resolution rising edge value"]
pub type SCR2_R = crate::FieldReader;
#[doc = "Field `SCR2` writer - High resolution rising edge value"]
pub type SCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&self) -> SCR2_R {
        SCR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    #[must_use]
    pub fn scr2(&mut self) -> SCR2_W<SCR2_SPEC> {
        SCR2_W::new(self, 0)
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
#[doc = "HRC shadow falling edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR2_SPEC;
impl crate::RegisterSpec for SCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2::R`](R) reader structure"]
impl crate::Readable for SCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr2::W`](W) writer structure"]
impl crate::Writable for SCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for SCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
