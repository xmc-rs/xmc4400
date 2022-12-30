#[doc = "Register `HDCLR` writer"]
pub struct W(crate::W<HDCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDCLR_SPEC>;
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
impl From<crate::W<HDCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up Pin Event Positive Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<EPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: EPEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Clear"]
pub type EPEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, EPEV_AW, O>;
impl<'a, const O: u8> EPEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE2)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<ENEV_AW> for bool {
    #[inline(always)]
    fn from(variant: ENEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Clear"]
pub type ENEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, ENEV_AW, O>;
impl<'a, const O: u8> ENEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE2)
    }
}
#[doc = "RTC Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<RTCEV_AW> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Clear"]
pub type RTCEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, RTCEV_AW, O>;
impl<'a, const O: u8> RTCEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE2)
    }
}
#[doc = "ULP WDG Alarm Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear watchdog alarm"]
    VALUE2 = 1,
}
impl From<ULPWDG_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Clear"]
pub type ULPWDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, ULPWDG_AW, O>;
impl<'a, const O: u8> ULPWDG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE1)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATPEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<VBATPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATPEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` writer - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
pub type VBATPEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, VBATPEV_AW, O>;
impl<'a, const O: u8> VBATPEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATPEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATPEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATNEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<VBATNEV_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATNEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` writer - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
pub type VBATNEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, VBATNEV_AW, O>;
impl<'a, const O: u8> VBATNEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATNEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATNEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0PEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0PEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0PEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type AHIBIO0PEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, AHIBIO0PEV_AW, O>;
impl<'a, const O: u8> AHIBIO0PEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0PEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0PEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0NEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0NEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0NEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type AHIBIO0NEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, AHIBIO0NEV_AW, O>;
impl<'a, const O: u8> AHIBIO0NEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0NEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0NEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1PEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO1PEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1PEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Clear"]
pub type AHIBIO1PEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, AHIBIO1PEV_AW, O>;
impl<'a, const O: u8> AHIBIO1PEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1PEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1PEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1NEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO1NEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1NEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Clear"]
pub type AHIBIO1NEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDCLR_SPEC, AHIBIO1NEV_AW, O>;
impl<'a, const O: u8> AHIBIO1NEV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1NEV_AW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1NEV_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EPEV_W<0> {
        EPEV_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> ENEV_W<1> {
        ENEV_W::new(self)
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RTCEV_W<2> {
        RTCEV_W::new(self)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> ULPWDG_W<3> {
        ULPWDG_W::new(self)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatpev(&mut self) -> VBATPEV_W<8> {
        VBATPEV_W::new(self)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatnev(&mut self) -> VBATNEV_W<9> {
        VBATNEV_W::new(self)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0pev(&mut self) -> AHIBIO0PEV_W<10> {
        AHIBIO0PEV_W::new(self)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0nev(&mut self) -> AHIBIO0NEV_W<11> {
        AHIBIO0NEV_W::new(self)
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1pev(&mut self) -> AHIBIO1PEV_W<12> {
        AHIBIO1PEV_W::new(self)
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1nev(&mut self) -> AHIBIO1NEV_W<13> {
        AHIBIO1NEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Domain Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdclr](index.html) module"]
pub struct HDCLR_SPEC;
impl crate::RegisterSpec for HDCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hdclr::W](W) writer structure"]
impl crate::Writable for HDCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HDCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
