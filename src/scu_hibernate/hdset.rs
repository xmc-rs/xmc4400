#[doc = "Register `HDSET` writer"]
pub struct W(crate::W<HDSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDSET_SPEC>;
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
impl From<crate::W<HDSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up Pin Event Positive Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct EPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Wake-up Pin Event Negative Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct ENEV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "RTC Event Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RTCEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "ULP WDG Alarm Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct ULPWDG_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE1)
    }
    #[doc = "Set watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct VBATPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATPEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATPEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATPEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATPEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct VBATNEV_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATNEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATNEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATNEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATNEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct AHIBIO0PEV_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0PEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0PEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0PEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0PEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct AHIBIO0NEV_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0NEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0NEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0NEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0NEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct AHIBIO1PEV_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO1PEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO1PEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1PEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1PEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct AHIBIO1NEV_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO1NEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO1NEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO1NEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO1NEV_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline(always)]
    pub fn epev(&mut self) -> EPEV_W {
        EPEV_W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline(always)]
    pub fn enev(&mut self) -> ENEV_W {
        ENEV_W { w: self }
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline(always)]
    pub fn rtcev(&mut self) -> RTCEV_W {
        RTCEV_W { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline(always)]
    pub fn ulpwdg(&mut self) -> ULPWDG_W {
        ULPWDG_W { w: self }
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    pub fn vbatpev(&mut self) -> VBATPEV_W {
        VBATPEV_W { w: self }
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Set"]
    #[inline(always)]
    pub fn vbatnev(&mut self) -> VBATNEV_W {
        VBATNEV_W { w: self }
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    pub fn ahibio0pev(&mut self) -> AHIBIO0PEV_W {
        AHIBIO0PEV_W { w: self }
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Set"]
    #[inline(always)]
    pub fn ahibio0nev(&mut self) -> AHIBIO0NEV_W {
        AHIBIO0NEV_W { w: self }
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    pub fn ahibio1pev(&mut self) -> AHIBIO1PEV_W {
        AHIBIO1PEV_W { w: self }
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing Set"]
    #[inline(always)]
    pub fn ahibio1nev(&mut self) -> AHIBIO1NEV_W {
        AHIBIO1NEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Domain Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdset](index.html) module"]
pub struct HDSET_SPEC;
impl crate::RegisterSpec for HDSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hdset::W](W) writer structure"]
impl crate::Writable for HDSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDSET to value 0"]
impl crate::Resettable for HDSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
