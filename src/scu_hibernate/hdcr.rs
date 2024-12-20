#[doc = "Register `HDCR` reader"]
pub type R = crate::R<HDCR_SPEC>;
#[doc = "Register `HDCR` writer"]
pub type W = crate::W<HDCR_SPEC>;
#[doc = "Wake-Up on Pin Event Positive Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKPEP_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<WKPEP_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEP` reader - Wake-Up on Pin Event Positive Edge Enable"]
pub type WKPEP_R = crate::BitReader<WKPEP_A>;
impl WKPEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKPEP_A {
        match self.bits {
            false => WKPEP_A::VALUE1,
            true => WKPEP_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WKPEP_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WKPEP_A::VALUE2
    }
}
#[doc = "Field `WKPEP` writer - Wake-Up on Pin Event Positive Edge Enable"]
pub type WKPEP_W<'a, REG> = crate::BitWriter<'a, REG, WKPEP_A>;
impl<'a, REG> WKPEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEP_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEP_A::VALUE2)
    }
}
#[doc = "Wake-up on Pin Event Negative Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKPEN_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<WKPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEN` reader - Wake-up on Pin Event Negative Edge Enable"]
pub type WKPEN_R = crate::BitReader<WKPEN_A>;
impl WKPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKPEN_A {
        match self.bits {
            false => WKPEN_A::VALUE1,
            true => WKPEN_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WKPEN_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WKPEN_A::VALUE2
    }
}
#[doc = "Field `WKPEN` writer - Wake-up on Pin Event Negative Edge Enable"]
pub type WKPEN_W<'a, REG> = crate::BitWriter<'a, REG, WKPEN_A>;
impl<'a, REG> WKPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEN_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WKPEN_A::VALUE2)
    }
}
#[doc = "Wake-up on RTC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCE_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<RTCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCE` reader - Wake-up on RTC Event Enable"]
pub type RTCE_R = crate::BitReader<RTCE_A>;
impl RTCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCE_A {
        match self.bits {
            false => RTCE_A::VALUE1,
            true => RTCE_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTCE_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTCE_A::VALUE2
    }
}
#[doc = "Field `RTCE` writer - Wake-up on RTC Event Enable"]
pub type RTCE_W<'a, REG> = crate::BitWriter<'a, REG, RTCE_A>;
impl<'a, REG> RTCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCE_A::VALUE2)
    }
}
#[doc = "ULP WDG Alarm Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGEN_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<ULPWDGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGEN` reader - ULP WDG Alarm Enable"]
pub type ULPWDGEN_R = crate::BitReader<ULPWDGEN_A>;
impl ULPWDGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULPWDGEN_A {
        match self.bits {
            false => ULPWDGEN_A::VALUE1,
            true => ULPWDGEN_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGEN_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGEN_A::VALUE2
    }
}
#[doc = "Field `ULPWDGEN` writer - ULP WDG Alarm Enable"]
pub type ULPWDGEN_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDGEN_A>;
impl<'a, REG> ULPWDGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGEN_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGEN_A::VALUE2)
    }
}
#[doc = "Hibernate Request Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_A {
    #[doc = "0: External hibernate request inactive"]
    VALUE1 = 0,
    #[doc = "1: External hibernate request active"]
    VALUE2 = 1,
}
impl From<HIB_A> for bool {
    #[inline(always)]
    fn from(variant: HIB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` reader - Hibernate Request Value Set"]
pub type HIB_R = crate::BitReader<HIB_A>;
impl HIB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIB_A {
        match self.bits {
            false => HIB_A::VALUE1,
            true => HIB_A::VALUE2,
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIB_A::VALUE1
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIB_A::VALUE2
    }
}
#[doc = "Field `HIB` writer - Hibernate Request Value Set"]
pub type HIB_W<'a, REG> = crate::BitWriter<'a, REG, HIB_A>;
impl<'a, REG> HIB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::VALUE1)
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::VALUE2)
    }
}
#[doc = "Multiplex Control for RTC_XTAL_1 Select as GPI Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTALGPI1SEL_A {
    #[doc = "0: RTC_XTAL_1 input selected"]
    VALUE1 = 0,
    #[doc = "1: Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    VALUE2 = 1,
}
impl From<XTALGPI1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: XTALGPI1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALGPI1SEL` reader - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
pub type XTALGPI1SEL_R = crate::BitReader<XTALGPI1SEL_A>;
impl XTALGPI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XTALGPI1SEL_A {
        match self.bits {
            false => XTALGPI1SEL_A::VALUE1,
            true => XTALGPI1SEL_A::VALUE2,
        }
    }
    #[doc = "RTC_XTAL_1 input selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XTALGPI1SEL_A::VALUE1
    }
    #[doc = "Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XTALGPI1SEL_A::VALUE2
    }
}
#[doc = "Field `XTALGPI1SEL` writer - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
pub type XTALGPI1SEL_W<'a, REG> = crate::BitWriter<'a, REG, XTALGPI1SEL_A>;
impl<'a, REG> XTALGPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_XTAL_1 input selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(XTALGPI1SEL_A::VALUE1)
    }
    #[doc = "Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(XTALGPI1SEL_A::VALUE2)
    }
}
#[doc = "fRTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCS_A {
    #[doc = "0: fOSI selected"]
    VALUE1 = 0,
    #[doc = "1: fULP selected"]
    VALUE2 = 1,
}
impl From<RCS_A> for bool {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCS` reader - fRTC Clock Selection"]
pub type RCS_R = crate::BitReader<RCS_A>;
impl RCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCS_A {
        match self.bits {
            false => RCS_A::VALUE1,
            true => RCS_A::VALUE2,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RCS_A::VALUE1
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RCS_A::VALUE2
    }
}
#[doc = "Field `RCS` writer - fRTC Clock Selection"]
pub type RCS_W<'a, REG> = crate::BitWriter<'a, REG, RCS_A>;
impl<'a, REG> RCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RCS_A::VALUE1)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RCS_A::VALUE2)
    }
}
#[doc = "fSTDBY Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STDBYSEL_A {
    #[doc = "0: fOSI selected"]
    VALUE1 = 0,
    #[doc = "1: fULP selected"]
    VALUE2 = 1,
}
impl From<STDBYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: STDBYSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBYSEL` reader - fSTDBY Clock Selection"]
pub type STDBYSEL_R = crate::BitReader<STDBYSEL_A>;
impl STDBYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STDBYSEL_A {
        match self.bits {
            false => STDBYSEL_A::VALUE1,
            true => STDBYSEL_A::VALUE2,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STDBYSEL_A::VALUE1
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STDBYSEL_A::VALUE2
    }
}
#[doc = "Field `STDBYSEL` writer - fSTDBY Clock Selection"]
pub type STDBYSEL_W<'a, REG> = crate::BitWriter<'a, REG, STDBYSEL_A>;
impl<'a, REG> STDBYSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STDBYSEL_A::VALUE1)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STDBYSEL_A::VALUE2)
    }
}
#[doc = "Wake-Up from Hibernate Trigger Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPSEL_A {
    #[doc = "0: HIB_IO_1 pin selected"]
    VALUE1 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    VALUE2 = 1,
}
impl From<WKUPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPSEL` reader - Wake-Up from Hibernate Trigger Input Selection"]
pub type WKUPSEL_R = crate::BitReader<WKUPSEL_A>;
impl WKUPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUPSEL_A {
        match self.bits {
            false => WKUPSEL_A::VALUE1,
            true => WKUPSEL_A::VALUE2,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WKUPSEL_A::VALUE1
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WKUPSEL_A::VALUE2
    }
}
#[doc = "Field `WKUPSEL` writer - Wake-Up from Hibernate Trigger Input Selection"]
pub type WKUPSEL_W<'a, REG> = crate::BitWriter<'a, REG, WKUPSEL_A>;
impl<'a, REG> WKUPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPSEL_A::VALUE1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPSEL_A::VALUE2)
    }
}
#[doc = "General Purpose Input 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPI0SEL_A {
    #[doc = "0: HIB_IO_1 pin selected"]
    VALUE1 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    VALUE2 = 1,
}
impl From<GPI0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPI0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPI0SEL` reader - General Purpose Input 0 Selection"]
pub type GPI0SEL_R = crate::BitReader<GPI0SEL_A>;
impl GPI0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPI0SEL_A {
        match self.bits {
            false => GPI0SEL_A::VALUE1,
            true => GPI0SEL_A::VALUE2,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GPI0SEL_A::VALUE1
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GPI0SEL_A::VALUE2
    }
}
#[doc = "Field `GPI0SEL` writer - General Purpose Input 0 Selection"]
pub type GPI0SEL_W<'a, REG> = crate::BitWriter<'a, REG, GPI0SEL_A>;
impl<'a, REG> GPI0SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GPI0SEL_A::VALUE1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GPI0SEL_A::VALUE2)
    }
}
#[doc = "General Purpose Input 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPI1SEL_A {
    #[doc = "0: HIB_IO_1 pin selected"]
    VALUE1 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    VALUE2 = 1,
}
impl From<GPI1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPI1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPI1SEL` reader - General Purpose Input 1 Selection"]
pub type GPI1SEL_R = crate::BitReader<GPI1SEL_A>;
impl GPI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPI1SEL_A {
        match self.bits {
            false => GPI1SEL_A::VALUE1,
            true => GPI1SEL_A::VALUE2,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GPI1SEL_A::VALUE1
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GPI1SEL_A::VALUE2
    }
}
#[doc = "Field `GPI1SEL` writer - General Purpose Input 1 Selection"]
pub type GPI1SEL_W<'a, REG> = crate::BitWriter<'a, REG, GPI1SEL_A>;
impl<'a, REG> GPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GPI1SEL_A::VALUE1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GPI1SEL_A::VALUE2)
    }
}
#[doc = "HIBIO0 Polarity Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBIO0POL_A {
    #[doc = "0: Direct value"]
    VALUE1 = 0,
    #[doc = "1: Inverted value"]
    VALUE2 = 1,
}
impl From<HIBIO0POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBIO0POL` reader - HIBIO0 Polarity Set"]
pub type HIBIO0POL_R = crate::BitReader<HIBIO0POL_A>;
impl HIBIO0POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBIO0POL_A {
        match self.bits {
            false => HIBIO0POL_A::VALUE1,
            true => HIBIO0POL_A::VALUE2,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO0POL_A::VALUE1
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO0POL_A::VALUE2
    }
}
#[doc = "Field `HIBIO0POL` writer - HIBIO0 Polarity Set"]
pub type HIBIO0POL_W<'a, REG> = crate::BitWriter<'a, REG, HIBIO0POL_A>;
impl<'a, REG> HIBIO0POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0POL_A::VALUE1)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0POL_A::VALUE2)
    }
}
#[doc = "HIBIO1 Polarity Set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBIO1POL_A {
    #[doc = "0: Direct value"]
    VALUE1 = 0,
    #[doc = "1: Inverted value"]
    VALUE2 = 1,
}
impl From<HIBIO1POL_A> for bool {
    #[inline(always)]
    fn from(variant: HIBIO1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBIO1POL` reader - HIBIO1 Polarity Set"]
pub type HIBIO1POL_R = crate::BitReader<HIBIO1POL_A>;
impl HIBIO1POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBIO1POL_A {
        match self.bits {
            false => HIBIO1POL_A::VALUE1,
            true => HIBIO1POL_A::VALUE2,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO1POL_A::VALUE1
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO1POL_A::VALUE2
    }
}
#[doc = "Field `HIBIO1POL` writer - HIBIO1 Polarity Set"]
pub type HIBIO1POL_W<'a, REG> = crate::BitWriter<'a, REG, HIBIO1POL_A>;
impl<'a, REG> HIBIO1POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1POL_A::VALUE1)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1POL_A::VALUE2)
    }
}
#[doc = "Select Analog Channel 0 or Digital Output Path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIG0SEL_A {
    #[doc = "0: Digital input"]
    VALUE1 = 0,
    #[doc = "1: Analog comparator result for HIB_IO_0"]
    VALUE2 = 1,
}
impl From<ADIG0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADIG0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIG0SEL` reader - Select Analog Channel 0 or Digital Output Path"]
pub type ADIG0SEL_R = crate::BitReader<ADIG0SEL_A>;
impl ADIG0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADIG0SEL_A {
        match self.bits {
            false => ADIG0SEL_A::VALUE1,
            true => ADIG0SEL_A::VALUE2,
        }
    }
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADIG0SEL_A::VALUE1
    }
    #[doc = "Analog comparator result for HIB_IO_0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADIG0SEL_A::VALUE2
    }
}
#[doc = "Field `ADIG0SEL` writer - Select Analog Channel 0 or Digital Output Path"]
pub type ADIG0SEL_W<'a, REG> = crate::BitWriter<'a, REG, ADIG0SEL_A>;
impl<'a, REG> ADIG0SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ADIG0SEL_A::VALUE1)
    }
    #[doc = "Analog comparator result for HIB_IO_0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ADIG0SEL_A::VALUE2)
    }
}
#[doc = "Select Analog Channel 1 or Digital Output Path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADIG1SEL_A {
    #[doc = "0: Digital input"]
    VALUE1 = 0,
    #[doc = "1: Analog comparator result for HIB_IO_1"]
    VALUE2 = 1,
}
impl From<ADIG1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADIG1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIG1SEL` reader - Select Analog Channel 1 or Digital Output Path"]
pub type ADIG1SEL_R = crate::BitReader<ADIG1SEL_A>;
impl ADIG1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADIG1SEL_A {
        match self.bits {
            false => ADIG1SEL_A::VALUE1,
            true => ADIG1SEL_A::VALUE2,
        }
    }
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADIG1SEL_A::VALUE1
    }
    #[doc = "Analog comparator result for HIB_IO_1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADIG1SEL_A::VALUE2
    }
}
#[doc = "Field `ADIG1SEL` writer - Select Analog Channel 1 or Digital Output Path"]
pub type ADIG1SEL_W<'a, REG> = crate::BitWriter<'a, REG, ADIG1SEL_A>;
impl<'a, REG> ADIG1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ADIG1SEL_A::VALUE1)
    }
    #[doc = "Analog comparator result for HIB_IO_1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ADIG1SEL_A::VALUE2)
    }
}
#[doc = "HIB_IO_0 Pin I/O Control (default HIBOUT)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIBIO0SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    VALUE1 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    VALUE2 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    VALUE3 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    VALUE4 = 8,
    #[doc = "9: Push-pull WDT service output"]
    VALUE5 = 9,
    #[doc = "10: Push-pull GPIO output"]
    VALUE6 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    VALUE7 = 12,
    #[doc = "13: Open-drain WDT service output"]
    VALUE8 = 13,
    #[doc = "14: Open-drain GPIO output"]
    VALUE9 = 14,
    #[doc = "15: Analog input"]
    VALUE10 = 15,
}
impl From<HIBIO0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO0SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIBIO0SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for HIBIO0SEL_A {}
#[doc = "Field `HIBIO0SEL` reader - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type HIBIO0SEL_R = crate::FieldReader<HIBIO0SEL_A>;
impl HIBIO0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HIBIO0SEL_A> {
        match self.bits {
            0 => Some(HIBIO0SEL_A::VALUE1),
            1 => Some(HIBIO0SEL_A::VALUE2),
            2 => Some(HIBIO0SEL_A::VALUE3),
            8 => Some(HIBIO0SEL_A::VALUE4),
            9 => Some(HIBIO0SEL_A::VALUE5),
            10 => Some(HIBIO0SEL_A::VALUE6),
            12 => Some(HIBIO0SEL_A::VALUE7),
            13 => Some(HIBIO0SEL_A::VALUE8),
            14 => Some(HIBIO0SEL_A::VALUE9),
            15 => Some(HIBIO0SEL_A::VALUE10),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE1
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE2
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE3
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE4
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE5
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE6
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE7
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE8
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE9
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == HIBIO0SEL_A::VALUE10
    }
}
#[doc = "Field `HIBIO0SEL` writer - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type HIBIO0SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HIBIO0SEL_A>;
impl<'a, REG> HIBIO0SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE1)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE2)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE3)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE4)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE5)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE6)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE7)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE8)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE9)
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO0SEL_A::VALUE10)
    }
}
#[doc = "HIB_IO_1 Pin I/O Control (Default WKUP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIBIO1SEL_A {
    #[doc = "0: Direct input, No input pull device connected"]
    VALUE1 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    VALUE2 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    VALUE3 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    VALUE4 = 8,
    #[doc = "9: Push-pull WDT service output"]
    VALUE5 = 9,
    #[doc = "10: Push-pull GPIO output"]
    VALUE6 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    VALUE7 = 12,
    #[doc = "13: Open-drain WDT service output"]
    VALUE8 = 13,
    #[doc = "14: Open-drain GPIO output"]
    VALUE9 = 14,
    #[doc = "15: Analog input"]
    VALUE10 = 15,
}
impl From<HIBIO1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIBIO1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIBIO1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for HIBIO1SEL_A {}
#[doc = "Field `HIBIO1SEL` reader - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type HIBIO1SEL_R = crate::FieldReader<HIBIO1SEL_A>;
impl HIBIO1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HIBIO1SEL_A> {
        match self.bits {
            0 => Some(HIBIO1SEL_A::VALUE1),
            1 => Some(HIBIO1SEL_A::VALUE2),
            2 => Some(HIBIO1SEL_A::VALUE3),
            8 => Some(HIBIO1SEL_A::VALUE4),
            9 => Some(HIBIO1SEL_A::VALUE5),
            10 => Some(HIBIO1SEL_A::VALUE6),
            12 => Some(HIBIO1SEL_A::VALUE7),
            13 => Some(HIBIO1SEL_A::VALUE8),
            14 => Some(HIBIO1SEL_A::VALUE9),
            15 => Some(HIBIO1SEL_A::VALUE10),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE1
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE2
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE3
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE4
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE5
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE6
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE7
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE8
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE9
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == HIBIO1SEL_A::VALUE10
    }
}
#[doc = "Field `HIBIO1SEL` writer - HIB_IO_1 Pin I/O Control (Default WKUP)"]
pub type HIBIO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HIBIO1SEL_A>;
impl<'a, REG> HIBIO1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE1)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE2)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE3)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE4)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE5)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE6)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE7)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE8)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE9)
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(HIBIO1SEL_A::VALUE10)
    }
}
#[doc = "Wake-Up on VBAT Falling Below Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATLO_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<VBATLO_A> for bool {
    #[inline(always)]
    fn from(variant: VBATLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATLO` reader - Wake-Up on VBAT Falling Below Threshold Enable"]
pub type VBATLO_R = crate::BitReader<VBATLO_A>;
impl VBATLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATLO_A {
        match self.bits {
            false => VBATLO_A::VALUE1,
            true => VBATLO_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATLO_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATLO_A::VALUE2
    }
}
#[doc = "Field `VBATLO` writer - Wake-Up on VBAT Falling Below Threshold Enable"]
pub type VBATLO_W<'a, REG> = crate::BitWriter<'a, REG, VBATLO_A>;
impl<'a, REG> VBATLO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATLO_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATLO_A::VALUE2)
    }
}
#[doc = "Wake-Up on VBAT Rising Above Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATHI_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<VBATHI_A> for bool {
    #[inline(always)]
    fn from(variant: VBATHI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATHI` reader - Wake-Up on VBAT Rising Above Threshold Enable"]
pub type VBATHI_R = crate::BitReader<VBATHI_A>;
impl VBATHI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATHI_A {
        match self.bits {
            false => VBATHI_A::VALUE1,
            true => VBATHI_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATHI_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATHI_A::VALUE2
    }
}
#[doc = "Field `VBATHI` writer - Wake-Up on VBAT Rising Above Threshold Enable"]
pub type VBATHI_W<'a, REG> = crate::BitWriter<'a, REG, VBATHI_A>;
impl<'a, REG> VBATHI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATHI_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATHI_A::VALUE2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0LO_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<AHIBIO0LO_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0LO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0LO` reader - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
pub type AHIBIO0LO_R = crate::BitReader<AHIBIO0LO_A>;
impl AHIBIO0LO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO0LO_A {
        match self.bits {
            false => AHIBIO0LO_A::VALUE1,
            true => AHIBIO0LO_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0LO_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0LO_A::VALUE2
    }
}
#[doc = "Field `AHIBIO0LO` writer - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
pub type AHIBIO0LO_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0LO_A>;
impl<'a, REG> AHIBIO0LO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0LO_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0LO_A::VALUE2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0HI_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<AHIBIO0HI_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0HI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0HI` reader - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
pub type AHIBIO0HI_R = crate::BitReader<AHIBIO0HI_A>;
impl AHIBIO0HI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO0HI_A {
        match self.bits {
            false => AHIBIO0HI_A::VALUE1,
            true => AHIBIO0HI_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0HI_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0HI_A::VALUE2
    }
}
#[doc = "Field `AHIBIO0HI` writer - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
pub type AHIBIO0HI_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0HI_A>;
impl<'a, REG> AHIBIO0HI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0HI_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0HI_A::VALUE2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_1 Falling Below Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1LO_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<AHIBIO1LO_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1LO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1LO` reader - Wake-Up on Analog HIB_IO_1 Falling Below Threshold Enable"]
pub type AHIBIO1LO_R = crate::BitReader<AHIBIO1LO_A>;
impl AHIBIO1LO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO1LO_A {
        match self.bits {
            false => AHIBIO1LO_A::VALUE1,
            true => AHIBIO1LO_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1LO_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1LO_A::VALUE2
    }
}
#[doc = "Field `AHIBIO1LO` writer - Wake-Up on Analog HIB_IO_1 Falling Below Threshold Enable"]
pub type AHIBIO1LO_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO1LO_A>;
impl<'a, REG> AHIBIO1LO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1LO_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1LO_A::VALUE2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_1 Rising Above Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO1HI_A {
    #[doc = "0: Wake-up event disabled"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event enabled"]
    VALUE2 = 1,
}
impl From<AHIBIO1HI_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO1HI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO1HI` reader - Wake-Up on Analog HIB_IO_1 Rising Above Threshold Enable"]
pub type AHIBIO1HI_R = crate::BitReader<AHIBIO1HI_A>;
impl AHIBIO1HI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHIBIO1HI_A {
        match self.bits {
            false => AHIBIO1HI_A::VALUE1,
            true => AHIBIO1HI_A::VALUE2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO1HI_A::VALUE1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO1HI_A::VALUE2
    }
}
#[doc = "Field `AHIBIO1HI` writer - Wake-Up on Analog HIB_IO_1 Rising Above Threshold Enable"]
pub type AHIBIO1HI_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO1HI_A>;
impl<'a, REG> AHIBIO1HI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1HI_A::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO1HI_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&self) -> WKPEP_R {
        WKPEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&self) -> WKPEN_R {
        WKPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&self) -> RTCE_R {
        RTCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&self) -> ULPWDGEN_R {
        ULPWDGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
    #[inline(always)]
    pub fn xtalgpi1sel(&self) -> XTALGPI1SEL_R {
        XTALGPI1SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&self) -> STDBYSEL_R {
        STDBYSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WKUPSEL_R {
        WKUPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&self) -> GPI0SEL_R {
        GPI0SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General Purpose Input 1 Selection"]
    #[inline(always)]
    pub fn gpi1sel(&self) -> GPI1SEL_R {
        GPI1SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&self) -> HIBIO0POL_R {
        HIBIO0POL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&self) -> HIBIO1POL_R {
        HIBIO1POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select Analog Channel 0 or Digital Output Path"]
    #[inline(always)]
    pub fn adig0sel(&self) -> ADIG0SEL_R {
        ADIG0SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select Analog Channel 1 or Digital Output Path"]
    #[inline(always)]
    pub fn adig1sel(&self) -> ADIG1SEL_R {
        ADIG1SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&self) -> HIBIO0SEL_R {
        HIBIO0SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&self) -> HIBIO1SEL_R {
        HIBIO1SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Wake-Up on VBAT Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VBATLO_R {
        VBATLO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-Up on VBAT Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn vbathi(&self) -> VBATHI_R {
        VBATHI_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> AHIBIO0LO_R {
        AHIBIO0LO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> AHIBIO0HI_R {
        AHIBIO0HI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-Up on Analog HIB_IO_1 Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn ahibio1lo(&self) -> AHIBIO1LO_R {
        AHIBIO1LO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-Up on Analog HIB_IO_1 Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn ahibio1hi(&self) -> AHIBIO1HI_R {
        AHIBIO1HI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&mut self) -> WKPEP_W<HDCR_SPEC> {
        WKPEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&mut self) -> WKPEN_W<HDCR_SPEC> {
        WKPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&mut self) -> RTCE_W<HDCR_SPEC> {
        RTCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&mut self) -> ULPWDGEN_W<HDCR_SPEC> {
        ULPWDGEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&mut self) -> HIB_W<HDCR_SPEC> {
        HIB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
    #[inline(always)]
    pub fn xtalgpi1sel(&mut self) -> XTALGPI1SEL_W<HDCR_SPEC> {
        XTALGPI1SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&mut self) -> RCS_W<HDCR_SPEC> {
        RCS_W::new(self, 6)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&mut self) -> STDBYSEL_W<HDCR_SPEC> {
        STDBYSEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&mut self) -> WKUPSEL_W<HDCR_SPEC> {
        WKUPSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&mut self) -> GPI0SEL_W<HDCR_SPEC> {
        GPI0SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - General Purpose Input 1 Selection"]
    #[inline(always)]
    pub fn gpi1sel(&mut self) -> GPI1SEL_W<HDCR_SPEC> {
        GPI1SEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&mut self) -> HIBIO0POL_W<HDCR_SPEC> {
        HIBIO0POL_W::new(self, 12)
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline(always)]
    pub fn hibio1pol(&mut self) -> HIBIO1POL_W<HDCR_SPEC> {
        HIBIO1POL_W::new(self, 13)
    }
    #[doc = "Bit 14 - Select Analog Channel 0 or Digital Output Path"]
    #[inline(always)]
    pub fn adig0sel(&mut self) -> ADIG0SEL_W<HDCR_SPEC> {
        ADIG0SEL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Select Analog Channel 1 or Digital Output Path"]
    #[inline(always)]
    pub fn adig1sel(&mut self) -> ADIG1SEL_W<HDCR_SPEC> {
        ADIG1SEL_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&mut self) -> HIBIO0SEL_W<HDCR_SPEC> {
        HIBIO0SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline(always)]
    pub fn hibio1sel(&mut self) -> HIBIO1SEL_W<HDCR_SPEC> {
        HIBIO1SEL_W::new(self, 20)
    }
    #[doc = "Bit 24 - Wake-Up on VBAT Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn vbatlo(&mut self) -> VBATLO_W<HDCR_SPEC> {
        VBATLO_W::new(self, 24)
    }
    #[doc = "Bit 25 - Wake-Up on VBAT Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn vbathi(&mut self) -> VBATHI_W<HDCR_SPEC> {
        VBATHI_W::new(self, 25)
    }
    #[doc = "Bit 26 - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0lo(&mut self) -> AHIBIO0LO_W<HDCR_SPEC> {
        AHIBIO0LO_W::new(self, 26)
    }
    #[doc = "Bit 27 - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0hi(&mut self) -> AHIBIO0HI_W<HDCR_SPEC> {
        AHIBIO0HI_W::new(self, 27)
    }
    #[doc = "Bit 28 - Wake-Up on Analog HIB_IO_1 Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn ahibio1lo(&mut self) -> AHIBIO1LO_W<HDCR_SPEC> {
        AHIBIO1LO_W::new(self, 28)
    }
    #[doc = "Bit 29 - Wake-Up on Analog HIB_IO_1 Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn ahibio1hi(&mut self) -> AHIBIO1HI_W<HDCR_SPEC> {
        AHIBIO1HI_W::new(self, 29)
    }
}
#[doc = "Hibernate Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDCR_SPEC;
impl crate::RegisterSpec for HDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdcr::R`](R) reader structure"]
impl crate::Readable for HDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdcr::W`](W) writer structure"]
impl crate::Writable for HDCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCR to value 0x000c_2000"]
impl crate::Resettable for HDCR_SPEC {
    const RESET_VALUE: u32 = 0x000c_2000;
}
