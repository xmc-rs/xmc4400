#[doc = "Register `HDSET` writer"]
pub type W = crate::W<HdsetSpec>;
#[doc = "Wake-up Pin Event Positive Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Epev> for bool {
    #[inline(always)]
    fn from(variant: Epev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Set"]
pub type EpevW<'a, REG> = crate::BitWriter<'a, REG, Epev>;
impl<'a, REG> EpevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Value2)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Enev> for bool {
    #[inline(always)]
    fn from(variant: Enev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Set"]
pub type EnevW<'a, REG> = crate::BitWriter<'a, REG, Enev>;
impl<'a, REG> EnevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Value2)
    }
}
#[doc = "RTC Event Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Rtcev> for bool {
    #[inline(always)]
    fn from(variant: Rtcev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Set"]
pub type RtcevW<'a, REG> = crate::BitWriter<'a, REG, Rtcev>;
impl<'a, REG> RtcevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Value2)
    }
}
#[doc = "ULP WDG Alarm Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdg {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set watchdog alarm"]
    Value2 = 1,
}
impl From<Ulpwdg> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Set"]
pub type UlpwdgW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdg>;
impl<'a, REG> UlpwdgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Value1)
    }
    #[doc = "Set watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Value2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatpev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Vbatpev> for bool {
    #[inline(always)]
    fn from(variant: Vbatpev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` writer - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set"]
pub type VbatpevW<'a, REG> = crate::BitWriter<'a, REG, Vbatpev>;
impl<'a, REG> VbatpevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatnev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Vbatnev> for bool {
    #[inline(always)]
    fn from(variant: Vbatnev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` writer - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set"]
pub type VbatnevW<'a, REG> = crate::BitWriter<'a, REG, Vbatnev>;
impl<'a, REG> VbatnevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatnev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatnev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0pev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Ahibio0pev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0pev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set"]
pub type Ahibio0pevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0pev>;
impl<'a, REG> Ahibio0pevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0pev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0pev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0nev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Ahibio0nev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0nev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set"]
pub type Ahibio0nevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0nev>;
impl<'a, REG> Ahibio0nevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0nev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0nev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio1pev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Ahibio1pev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio1pev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set"]
pub type Ahibio1pevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio1pev>;
impl<'a, REG> Ahibio1pevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio1pev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio1pev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio1nev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set wake-up event"]
    Value2 = 1,
}
impl From<Ahibio1nev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio1nev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set"]
pub type Ahibio1nevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio1nev>;
impl<'a, REG> Ahibio1nevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio1nev::Value1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio1nev::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EpevW<HdsetSpec> {
        EpevW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> EnevW<HdsetSpec> {
        EnevW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RtcevW<HdsetSpec> {
        RtcevW::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> UlpwdgW<HdsetSpec> {
        UlpwdgW::new(self, 3)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatpev(&mut self) -> VbatpevW<HdsetSpec> {
        VbatpevW::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatnev(&mut self) -> VbatnevW<HdsetSpec> {
        VbatnevW::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0pev(&mut self) -> Ahibio0pevW<HdsetSpec> {
        Ahibio0pevW::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0nev(&mut self) -> Ahibio0nevW<HdsetSpec> {
        Ahibio0nevW::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1pev(&mut self) -> Ahibio1pevW<HdsetSpec> {
        Ahibio1pevW::new(self, 12)
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio1nev(&mut self) -> Ahibio1nevW<HdsetSpec> {
        Ahibio1nevW::new(self, 13)
    }
}
#[doc = "Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdsetSpec;
impl crate::RegisterSpec for HdsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdset::W`](W) writer structure"]
impl crate::Writable for HdsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDSET to value 0"]
impl crate::Resettable for HdsetSpec {
    const RESET_VALUE: u32 = 0;
}
