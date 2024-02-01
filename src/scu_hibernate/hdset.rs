#[doc = "Register `HDSET` writer"]
pub type W = crate::W<HDSET_SPEC>;
#[doc = "Wake-up Pin Event Positive Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<EPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: EPEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Set"]
pub type EPEV_W<'a, REG> = crate::BitWriter<'a, REG, EPEV_AW>;
impl<'a, REG> EPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPEV_AW::VALUE2)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<ENEV_AW> for bool {
    #[inline(always)]
    fn from(variant: ENEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Set"]
pub type ENEV_W<'a, REG> = crate::BitWriter<'a, REG, ENEV_AW>;
impl<'a, REG> ENEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENEV_AW::VALUE2)
    }
}
#[doc = "RTC Event Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<RTCEV_AW> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Set"]
pub type RTCEV_W<'a, REG> = crate::BitWriter<'a, REG, RTCEV_AW>;
impl<'a, REG> RTCEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEV_AW::VALUE2)
    }
}
#[doc = "ULP WDG Alarm Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set watchdog alarm"]
    VALUE2 = 1,
}
impl From<ULPWDG_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Set"]
pub type ULPWDG_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDG_AW>;
impl<'a, REG> ULPWDG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_AW::VALUE1)
    }
    #[doc = "Set watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATPEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<VBATPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATPEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` writer - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set"]
pub type VBATPEV_W<'a, REG> = crate::BitWriter<'a, REG, VBATPEV_AW>;
impl<'a, REG> VBATPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATPEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATPEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATNEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<VBATNEV_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATNEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` writer - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set"]
pub type VBATNEV_W<'a, REG> = crate::BitWriter<'a, REG, VBATNEV_AW>;
impl<'a, REG> VBATNEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATNEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATNEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0PEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0PEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0PEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set"]
pub type AHIBIO0PEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0PEV_AW>;
impl<'a, REG> AHIBIO0PEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0PEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0PEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0NEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0NEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0NEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set"]
pub type AHIBIO0NEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0NEV_AW>;
impl<'a, REG> AHIBIO0NEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0NEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0NEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1PEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO1PEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1PEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set"]
pub type AHIBIO1PEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO1PEV_AW>;
impl<'a, REG> AHIBIO1PEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1PEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1PEV_AW::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1NEV_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO1NEV_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1NEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set"]
pub type AHIBIO1NEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO1NEV_AW>;
impl<'a, REG> AHIBIO1NEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1NEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1NEV_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EPEV_W<HDSET_SPEC> {
        EPEV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> ENEV_W<HDSET_SPEC> {
        ENEV_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RTCEV_W<HDSET_SPEC> {
        RTCEV_W::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> ULPWDG_W<HDSET_SPEC> {
        ULPWDG_W::new(self, 3)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatpev(&mut self) -> VBATPEV_W<HDSET_SPEC> {
        VBATPEV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatnev(&mut self) -> VBATNEV_W<HDSET_SPEC> {
        VBATNEV_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0pev(&mut self) -> AHIBIO0PEV_W<HDSET_SPEC> {
        AHIBIO0PEV_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0nev(&mut self) -> AHIBIO0NEV_W<HDSET_SPEC> {
        AHIBIO0NEV_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1pev(&mut self) -> AHIBIO1PEV_W<HDSET_SPEC> {
        AHIBIO1PEV_W::new(self, 12)
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1nev(&mut self) -> AHIBIO1NEV_W<HDSET_SPEC> {
        AHIBIO1NEV_W::new(self, 13)
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
#[doc = "Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDSET_SPEC;
impl crate::RegisterSpec for HDSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdset::W`](W) writer structure"]
impl crate::Writable for HDSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDSET to value 0"]
impl crate::Resettable for HDSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
