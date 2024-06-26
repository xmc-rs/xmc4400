#[doc = "Register `HDSTAT` reader"]
pub type R = crate::R<HDSTAT_SPEC>;
#[doc = "Wake-up Pin Event Positive Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_A {
    #[doc = "0: Wake-up on positive edge pin event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on positive edge pin event active"]
    VALUE2 = 1,
}
impl From<EPEV_A> for bool {
    #[inline(always)]
    fn from(variant: EPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` reader - Wake-up Pin Event Positive Edge"]
pub type EPEV_R = crate::BitReader<EPEV_A>;
impl EPEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPEV_A {
        match self.bits {
            false => EPEV_A::VALUE1,
            true => EPEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on positive edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPEV_A::VALUE1
    }
    #[doc = "Wake-up on positive edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPEV_A::VALUE2
    }
}
#[doc = "Wake-up Pin Event Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_A {
    #[doc = "0: Wake-up on negative edge pin event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on negative edge pin event active"]
    VALUE2 = 1,
}
impl From<ENEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` reader - Wake-up Pin Event Negative Edge"]
pub type ENEV_R = crate::BitReader<ENEV_A>;
impl ENEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENEV_A {
        match self.bits {
            false => ENEV_A::VALUE1,
            true => ENEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on negative edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENEV_A::VALUE1
    }
    #[doc = "Wake-up on negative edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENEV_A::VALUE2
    }
}
#[doc = "RTC Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_A {
    #[doc = "0: Wake-up on RTC event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on RTC event active"]
    VALUE2 = 1,
}
impl From<RTCEV_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` reader - RTC Event"]
pub type RTCEV_R = crate::BitReader<RTCEV_A>;
impl RTCEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCEV_A {
        match self.bits {
            false => RTCEV_A::VALUE1,
            true => RTCEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on RTC event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTCEV_A::VALUE1
    }
    #[doc = "Wake-up on RTC event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTCEV_A::VALUE2
    }
}
#[doc = "ULP WDG Alarm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_A {
    #[doc = "0: Watchdog alarm did not occur"]
    VALUE1 = 0,
    #[doc = "1: Watchdog alarm occurred"]
    VALUE2 = 1,
}
impl From<ULPWDG_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` reader - ULP WDG Alarm Status"]
pub type ULPWDG_R = crate::BitReader<ULPWDG_A>;
impl ULPWDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULPWDG_A {
        match self.bits {
            false => ULPWDG_A::VALUE1,
            true => ULPWDG_A::VALUE2,
        }
    }
    #[doc = "Watchdog alarm did not occur"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDG_A::VALUE1
    }
    #[doc = "Watchdog alarm occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDG_A::VALUE2
    }
}
#[doc = "Hibernate Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNOUT_A {
    #[doc = "0: Hibernate not driven active to pads"]
    VALUE1 = 0,
    #[doc = "1: Hibernate driven active to pads"]
    VALUE2 = 1,
}
impl From<HIBNOUT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNOUT` reader - Hibernate Control Status"]
pub type HIBNOUT_R = crate::BitReader<HIBNOUT_A>;
impl HIBNOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBNOUT_A {
        match self.bits {
            false => HIBNOUT_A::VALUE1,
            true => HIBNOUT_A::VALUE2,
        }
    }
    #[doc = "Hibernate not driven active to pads"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBNOUT_A::VALUE1
    }
    #[doc = "Hibernate driven active to pads"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBNOUT_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATPEV_A {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    VALUE2 = 1,
}
impl From<VBATPEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` reader - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
pub type VBATPEV_R = crate::BitReader<VBATPEV_A>;
impl VBATPEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATPEV_A {
        match self.bits {
            false => VBATPEV_A::VALUE1,
            true => VBATPEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on rising above threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATPEV_A::VALUE1
    }
    #[doc = "Wake-up on rising above threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATPEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATNEV_A {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    VALUE2 = 1,
}
impl From<VBATNEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATNEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` reader - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
pub type VBATNEV_R = crate::BitReader<VBATNEV_A>;
impl VBATNEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATNEV_A {
        match self.bits {
            false => VBATNEV_A::VALUE1,
            true => VBATNEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on falling below threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATNEV_A::VALUE1
    }
    #[doc = "Wake-up on falling below threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATNEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0PEV_A {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO0PEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0PEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` reader - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
pub type AHIBIO0PEV_R = crate::BitReader<AHIBIO0PEV_A>;
impl AHIBIO0PEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO0PEV_A {
        match self.bits {
            false => AHIBIO0PEV_A::VALUE1,
            true => AHIBIO0PEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on rising above threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0PEV_A::VALUE1
    }
    #[doc = "Wake-up on rising above threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0PEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0NEV_A {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO0NEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0NEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` reader - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
pub type AHIBIO0NEV_R = crate::BitReader<AHIBIO0NEV_A>;
impl AHIBIO0NEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO0NEV_A {
        match self.bits {
            false => AHIBIO0NEV_A::VALUE1,
            true => AHIBIO0NEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on falling below threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0NEV_A::VALUE1
    }
    #[doc = "Wake-up on falling below threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0NEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1PEV_A {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO1PEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1PEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1PEV` reader - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing"]
pub type AHIBIO1PEV_R = crate::BitReader<AHIBIO1PEV_A>;
impl AHIBIO1PEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO1PEV_A {
        match self.bits {
            false => AHIBIO1PEV_A::VALUE1,
            true => AHIBIO1PEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on rising above threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1PEV_A::VALUE1
    }
    #[doc = "Wake-up on rising above threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1PEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1NEV_A {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO1NEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1NEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1NEV` reader - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing"]
pub type AHIBIO1NEV_R = crate::BitReader<AHIBIO1NEV_A>;
impl AHIBIO1NEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO1NEV_A {
        match self.bits {
            false => AHIBIO1NEV_A::VALUE1,
            true => AHIBIO1NEV_A::VALUE2,
        }
    }
    #[doc = "Wake-up on falling below threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1NEV_A::VALUE1
    }
    #[doc = "Wake-up on falling below threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1NEV_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline(always)]
    pub fn epev(&self) -> EPEV_R {
        EPEV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline(always)]
    pub fn enev(&self) -> ENEV_R {
        ENEV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline(always)]
    pub fn rtcev(&self) -> RTCEV_R {
        RTCEV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline(always)]
    pub fn ulpwdg(&self) -> ULPWDG_R {
        ULPWDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline(always)]
    pub fn hibnout(&self) -> HIBNOUT_R {
        HIBNOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatpev(&self) -> VBATPEV_R {
        VBATPEV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatnev(&self) -> VBATNEV_R {
        VBATNEV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0pev(&self) -> AHIBIO0PEV_R {
        AHIBIO0PEV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0nev(&self) -> AHIBIO0NEV_R {
        AHIBIO0NEV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-Up on LPAC Positive Edge of HIB_IO_1 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio1pev(&self) -> AHIBIO1PEV_R {
        AHIBIO1PEV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake-Up on LPAC Negative Edge of HIB_IO_1 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio1nev(&self) -> AHIBIO1NEV_R {
        AHIBIO1NEV_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Hibernate Domain Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDSTAT_SPEC;
impl crate::RegisterSpec for HDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdstat::R`](R) reader structure"]
impl crate::Readable for HDSTAT_SPEC {}
#[doc = "`reset()` method sets HDSTAT to value 0"]
impl crate::Resettable for HDSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
