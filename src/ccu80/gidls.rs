#[doc = "Register `GIDLS` writer"]
pub type W = crate::W<GIDLS_SPEC>;
#[doc = "Field `SS0I` writer - CC80 IDLE mode set"]
pub type SS0I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS1I` writer - CC81 IDLE mode set"]
pub type SS1I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS2I` writer - CC82 IDLE mode set"]
pub type SS2I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS3I` writer - CC83 IDLE mode set"]
pub type SS3I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPRB` writer - Prescaler# Run Bit Clear"]
pub type CPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIC` writer - Prescaler clear"]
pub type PSIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCH` writer - Parity Checker Run bit clear"]
pub type CPCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss0i(&mut self) -> SS0I_W<GIDLS_SPEC> {
        SS0I_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC81 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss1i(&mut self) -> SS1I_W<GIDLS_SPEC> {
        SS1I_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC82 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss2i(&mut self) -> SS2I_W<GIDLS_SPEC> {
        SS2I_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC83 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss3i(&mut self) -> SS3I_W<GIDLS_SPEC> {
        SS3I_W::new(self, 3)
    }
    #[doc = "Bit 8 - Prescaler# Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cprb(&mut self) -> CPRB_W<GIDLS_SPEC> {
        CPRB_W::new(self, 8)
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    #[must_use]
    pub fn psic(&mut self) -> PSIC_W<GIDLS_SPEC> {
        PSIC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity Checker Run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpch(&mut self) -> CPCH_W<GIDLS_SPEC> {
        CPCH_W::new(self, 10)
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
#[doc = "Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIDLS_SPEC;
impl crate::RegisterSpec for GIDLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidls::W`](W) writer structure"]
impl crate::Writable for GIDLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIDLS to value 0"]
impl crate::Resettable for GIDLS_SPEC {
    const RESET_VALUE: u32 = 0;
}
